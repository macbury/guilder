use entity::{QueryOrder, Wallet, wallets, EntityTrait};
use rocket::serde::json::Json;
use sea_orm_rocket::Connection;
use serde::Serialize;
use super::types::WalletResource;

use crate::{
  stages::{
    session::CurrentSession,
    Db
  },
  env::{ResponseResult, catch_error},
};

#[derive(Serialize, Debug)]
pub struct AllResponse {
  wallets: Vec<WalletResource>
}

#[tracing::instrument(level="info", skip(conn, _session))]
#[get("/")]
pub async fn action<'a>(conn: Connection<'a, Db>, _session : CurrentSession) -> ResponseResult<Json<AllResponse>> {
  let db = conn.into_inner();
  let wallets : Vec<WalletResource> = Wallet::find()
    .find_also_related(entity::WalletMetadata)
    .order_by_asc(wallets::Column::Name)
    .all(db)
    .await
    .map_err(catch_error)?
    .iter()
    .map(|models| models.into())
    .collect();

  tracing::info!("Found: {} wallets", wallets.len());

  Ok(Json(AllResponse { wallets }))
}

#[cfg(test)]
mod test {
  use rocket::{http::Status, local::asynchronous::{Client, LocalResponse}};
  use crate::test;

  async fn response<'a>(client : &'a Client) -> LocalResponse<'a> {
    let response = client.get(format!("/api/wallets"))
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
      "errors": vec!["You need to sign in to access: /api/wallets"],
      "status": "forbidden"
    }).await;
  }
}
