use entity::{Wallet, EntityTrait};
use rocket::{serde::{Serialize, json::{Json}}};
use sea_orm_rocket::Connection;
use crate::{stages::{Db, session::CurrentSession}, env::{ResponseResult, self}};

#[derive(Serialize, Debug)]
pub struct DestroyWalletResponse {
  deleted: u64
}

#[tracing::instrument(level="info", skip(conn, _session))]
#[delete("/<wallet_id>")]
pub async fn action<'a>(_session : CurrentSession, wallet_id: i64, conn: Connection<'a, Db>) -> ResponseResult<Json<DestroyWalletResponse>> {
  let deleted = Wallet::delete_by_id(wallet_id)
    .exec(conn.into_inner())
    .await
    .map_err(env::catch_error)?;

  Ok(Json(DestroyWalletResponse { deleted: deleted.rows_affected }))
}


#[cfg(test)]
mod test {
  use rocket::{http::Status, local::asynchronous::{Client, LocalResponse}};
  use crate::test;

  async fn response<'a>(client : &'a Client) -> LocalResponse<'a> {
    let response = client.delete(format!("/api/wallets/1"))
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
