use entity::{Bond, EntityTrait, ModelTrait};
use rocket::{serde::{Serialize, json::Json}};
use sea_orm_rocket::Connection;
use crate::{stages::{Db, session::CurrentSession}, env::{ResponseResult, self}};

#[derive(Serialize, Debug)]
pub struct DestroyResponse {
  deleted: u64
}

#[tracing::instrument(level="info", skip(conn, _session))]
#[delete("/<id>")]
pub async fn action<'a>(_session : CurrentSession, id : i64, conn: Connection<'a, Db>) -> ResponseResult<Option<Json<DestroyResponse>>> {
  let db = conn.into_inner();
  let bond = Bond::find_by_id(id)
    .one(db)
    .await
    .map_err(env::catch_error)?;

  if bond.is_none() {
    tracing::info!("Asset is not found");
    return Ok(None)
  }

  let bond = bond.unwrap();
  let result = bond.delete(db).await
    .map_err(env::catch_error)?;
  tracing::info!("Deleted rows: {:?}", result);

  Ok(Some(Json(DestroyResponse { deleted: result.rows_affected })))
}

#[cfg(test)]
mod test {
  use rocket::{http::Status, local::asynchronous::{Client, LocalResponse}};
  use crate::test;

  async fn response<'a>(client : &'a Client) -> LocalResponse<'a> {
    let response = client.delete(format!("/api/bonds/1"))
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
      "errors": vec!["You need to sign in to access: /api/bonds/1"],
      "status": "forbidden"
    }).await;
  }
}
