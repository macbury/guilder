use entity::{QueryOrder, EntityTrait, IntegrationModel, Integration, integrations};
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
pub struct AllResponse {
  integrations: Vec<IntegrationModel>
}

#[tracing::instrument(level="info", skip(conn, _session))]
#[get("/")]
pub async fn action<'a>(conn: Connection<'a, Db>, _session : CurrentSession) -> ResponseResult<Json<AllResponse>> {
  let db = conn.into_inner();
  let integrations = Integration::find()
    .order_by_asc(integrations::Column::Name)
    .all(db)
    .await
    .map_err(catch_error)?;

  tracing::info!("Found: {} integrations", integrations.len());

  Ok(Json(AllResponse { integrations }))
}

#[cfg(test)]
mod test {
  use entity::{integrations, ActiveValue::NotSet, Set, ActiveModelTrait};
  use rocket::{http::Status, local::asynchronous::{Client, LocalResponse}};
  use crate::test;

  async fn list_all<'a>(client : &'a Client) -> LocalResponse<'a> {
    let response = client.get("/api/integrations")
      .dispatch()
      .await;

    return response;
  }

  #[rocket::async_test]
  async fn reject_guest() {
    let (client, _db) = test::server().await;
    let response = list_all(&client).await;

    assert_eq!(response.status(), Status::Forbidden);
    assert_body!(response, {
      "errors": vec!["You need to sign in to access: /api/integrations"],
      "status": "forbidden"
    }).await;
  }

  #[rocket::async_test]
  async fn return_list_of_resources() -> anyhow::Result<()> {
    let (client, db) = test::server().await;
    let (_user, _cookie) = test::sign_in(&client, &db).await;
    let integration = integrations::ActiveModel {
      id: NotSet,
      name: Set("Hello world".to_owned()),
      kind: Set(integrations::Kind::PKOBP),
      login: Set("".to_owned()),
      password: Set(None),
      ..Default::default()
    };

    let integration = integration.insert(&db).await?;
    let response = list_all(&client).await;

    assert_eq!(response.status(), Status::Ok);
    assert_body!(response, {
      "integrations": [{
        "id": integration.id,
        "name": "Hello world",
        "status": "Initializing",
        "last_sync_at": null,
        "kind": "PKOBP",
        "login": ""
      }]
    }).await;

    Ok(())
  }
}
