use entity::{Integration, EntityTrait};
use rocket::{serde::json::Json, State};
use sea_orm_rocket::Connection;
use serde::Serialize;

use crate::{
  stages::{
    session::CurrentSession,
    Db,
    pkobp_integration::StatePKOBondsImporter
  },
  env::{ResponseResult, catch_error},
};

#[derive(Serialize, Debug)]
pub struct SyncResponse {
  success: bool
}

#[tracing::instrument(level="info", skip(conn, _session, importer))]
#[post("/<id>/sync")]
pub async fn action<'a>(conn: Connection<'a, Db>, _session : CurrentSession, id: i64, importer : &State<StatePKOBondsImporter>) -> ResponseResult<Option<Json<SyncResponse>>> {
  let db = conn.into_inner();
  let integration = Integration::find_by_id(id)
    .one(db)
    .await
    .map_err(catch_error)?;

  if let Some(integration) = integration {
    tracing::debug!("Locking...");
    let importer = importer.write().await;
    tracing::debug!("Importer ready, syncing...");
    let result = importer.sync(&integration, true).await;
    tracing::debug!("Sync finished with: {:?}", result);
    return Ok(Some(Json(SyncResponse { success: result.is_ok() })))
  }

  Ok(None)
}

#[cfg(test)]
mod test {
  use rocket::{http::Status, local::asynchronous::{Client, LocalResponse}};
  use crate::test;

  async fn request<'a>(client : &'a Client, id: i64) -> LocalResponse<'a> {
    let response = client.post(format!("/api/integrations/{}/sync", id))
      .dispatch()
      .await;

    return response;
  }

  #[rocket::async_test]
  async fn reject_guest() {
    let (client, _db) = test::server().await;
    let response = request(&client, 1).await;

    assert_eq!(response.status(), Status::Forbidden);
    assert_body!(response, {
      "errors": vec!["You need to sign in to access: /api/integrations/1/sync"],
      "status": "forbidden"
    }).await;
  }
}
