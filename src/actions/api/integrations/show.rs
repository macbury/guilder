use entity::{IntegrationModel, Integration, EntityTrait};
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

#[derive(Serialize, Debug)]
pub struct ShowResponse {
  integration: IntegrationModel
}

#[tracing::instrument(level="info", skip(conn, _session))]
#[get("/<id>")]
pub async fn action<'a>(conn: Connection<'a, Db>, _session : CurrentSession, id: i64) -> ResponseResult<Option<Json<ShowResponse>>> {
  let db = conn.into_inner();
  let integration = Integration::find_by_id(id)
    .one(db)
    .await
    .map_err(catch_error)?
    .map(|integration| Json(ShowResponse { integration }));

  Ok(integration)
}

#[cfg(test)]
mod test {
  use rocket::{http::Status, local::asynchronous::{Client, LocalResponse}};
  use crate::test;

  async fn request<'a>(client : &'a Client, id: i64) -> LocalResponse<'a> {
    let response = client.get(format!("/api/integrations/{}", id))
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
      "errors": vec!["You need to sign in to access: /api/integrations/1"],
      "status": "forbidden"
    }).await;
  }
}
