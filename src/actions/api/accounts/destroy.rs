use entity::{Account, EntityTrait};
use rocket::{serde::{Serialize, json::{Json}}};
use sea_orm_rocket::Connection;
use crate::{stages::{Db, session::CurrentSession}, env::{ResponseResult, self}};

#[derive(Serialize, Debug)]
pub struct DestroyAccountResponse {
  deleted: u64
}

#[tracing::instrument(level="info", skip(conn, _session))]
#[delete("/<account_id>")]
pub async fn action<'a>(_session : CurrentSession, account_id: i64, conn: Connection<'a, Db>) -> ResponseResult<Json<DestroyAccountResponse>> {
  let deleted = Account::delete_by_id(account_id)
    .exec(conn.into_inner())
    .await
    .map_err(env::catch_error)?;

  Ok(Json(DestroyAccountResponse { deleted: deleted.rows_affected }))
}


#[cfg(test)]
mod test {
  use rocket::{http::Status, local::asynchronous::{Client, LocalResponse}};
  use crate::test;

  async fn response<'a>(client : &'a Client) -> LocalResponse<'a> {
    let response = client.delete(format!("/api/accounts/1"))
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
