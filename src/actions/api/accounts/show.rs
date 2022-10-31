use entity::{Account, EntityTrait, AccountMetadata};
use rocket::serde::json::Json;
use sea_orm_rocket::Connection;
use serde::Serialize;

use crate::{
  stages::{
    session::CurrentSession,
    Db
  },
  env::{ResponseResult, catch_error},
};

use super::types::AccountResource;

#[derive(Serialize, Debug)]
pub struct ShowAccountResponse {
  account: AccountResource
}

#[tracing::instrument(level="info", skip(conn, _session))]
#[get("/<account_id>")]
pub async fn action<'a>(conn: Connection<'a, Db>, _session : CurrentSession, account_id: i64) -> ResponseResult<Option<Json<ShowAccountResponse>>> {
  let db = conn.into_inner();
  let account = Account::find_by_id(account_id)
    .find_also_related(AccountMetadata)
    .one(db)
    .await
    .map_err(catch_error)?
    .map(|account| Json(ShowAccountResponse { account: account.into() }));

  tracing::info!("Found account: {:?}", account);
  Ok(account)
}

#[cfg(test)]
mod test {
  use rocket::{http::Status, local::asynchronous::{Client, LocalResponse}};
  use crate::test;

  async fn response<'a>(client : &'a Client) -> LocalResponse<'a> {
    let response = client.get(format!("/api/accounts/1"))
      .dispatch()
      .await;

    return response;
  }

  #[rocket::async_test]
  async fn reject_guest() {
    let (client, _db) = test::server().await;
    let response = response(&client).await;

    assert_eq!(response.status(), Status::Forbidden);
    assert_body!(response, {
      "errors": vec!["You need to sign in to access: /api/accounts/1"],
      "status": "forbidden"
    }).await;
  }
}
