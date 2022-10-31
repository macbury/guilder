use entity::{Wallet, EntityTrait};
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

use super::types::WalletResource;

#[derive(Serialize, Debug)]
pub struct ShowResponse {
  wallet: WalletResource
}

#[tracing::instrument(level="info", skip(conn, _session))]
#[get("/<wallet_id>")]
pub async fn action<'a>(conn: Connection<'a, Db>, _session : CurrentSession, wallet_id: i64) -> ResponseResult<Option<Json<ShowResponse>>> {
  let db = conn.into_inner();
  let wallet = Wallet::find_by_id(wallet_id)
    .find_also_related(entity::WalletMetadata)
    .one(db)
    .await
    .map_err(catch_error)?
    .map(|wallet| Json(ShowResponse { wallet: wallet.into() }));

  tracing::info!("Found wallet: {:?}", wallet);
  Ok(wallet)
}

#[cfg(test)]
mod test {
  use rocket::{http::Status, local::asynchronous::{Client, LocalResponse}};
  use crate::test;

  async fn response<'a>(client : &'a Client) -> LocalResponse<'a> {
    let response = client.get(format!("/api/wallets/1"))
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
      "errors": vec!["You need to sign in to access: /api/wallets/1"],
      "status": "forbidden"
    }).await;
  }
}
