use entity::{QueryOrder, Account, accounts, EntityTrait, AccountMetadata};
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
pub struct AllResponse {
  accounts: Vec<AccountResource>
}

#[tracing::instrument(level="info", skip(conn, _session))]
#[get("/")]
pub async fn action<'a>(conn: Connection<'a, Db>, _session : CurrentSession) -> ResponseResult<Json<AllResponse>> {
  let db = conn.into_inner();
  let accounts : Vec<AccountResource> = Account::find()
    .find_also_related(AccountMetadata)
    .order_by_asc(accounts::Column::Name)
    .all(db)
    .await
    .map_err(catch_error)?
    .iter()
    .map(|models| models.into())
    .collect();

  tracing::info!("Found: {} accounts", accounts.len());

  Ok(Json(AllResponse { accounts }))
}

#[cfg(test)]
mod test {
  use rocket::{http::Status, local::asynchronous::{Client, LocalResponse}};
  use crate::test;

  async fn response<'a>(client : &'a Client) -> LocalResponse<'a> {
    let response = client.get(format!("/api/accounts"))
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
      "errors": vec!["You need to sign in to access: /api/accounts"],
      "status": "forbidden"
    }).await;
  }
}
