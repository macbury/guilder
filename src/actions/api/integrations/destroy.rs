use entity::{Integration, EntityTrait};
use rocket::{serde::{Serialize, json::{Json}}};
use sea_orm_rocket::Connection;
use crate::{stages::{Db, session::CurrentSession}, env::{ResponseResult, self}};

#[derive(Serialize, Debug)]
pub struct DestroyResponse {
  deleted: u64
}

#[tracing::instrument(level="info", skip(conn, _session))]
#[delete("/<id>")]
pub async fn action<'a>(_session : CurrentSession, id: i64, conn: Connection<'a, Db>) -> ResponseResult<Json<DestroyResponse>> {
  let deleted = Integration::delete_by_id(id)
    .exec(conn.into_inner())
    .await
    .map_err(env::catch_error)?;

  Ok(Json(DestroyResponse { deleted: deleted.rows_affected }))
}

#[cfg(test)]
mod test {
  use rocket::{http::Status, local::asynchronous::{Client, LocalResponse}};
  use crate::test;

  async fn destroy<'a>(client : &'a Client, id: i64) -> LocalResponse<'a> {
    let response = client.delete(format!("/api/integrations/{}", id))
      .dispatch()
      .await;

    return response;
  }

  #[rocket::async_test]
  async fn reject_guest() {
    let (client, _db) = test::server().await;
    let response = destroy(&client, 1).await;

    assert_eq!(response.status(), Status::Forbidden);
    assert_body!(response, {
      "errors": vec!["You need to sign in to access: /api/integrations/1"],
      "status": "forbidden"
    }).await;
  }
}
