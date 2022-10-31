use entity::{integrations, ActiveModelTrait, IntegrationModel, ActiveValue::NotSet, Set};
use rocket::State;
use sea_orm_rocket::Connection;
use super::types::JsonIntegrationsParams;
use crate::{
  stages::{
    Db,
    session::CurrentSession
  },
  env::{
    self,
    CrudResponse,
    CrudResult,
    GuilderConfig
  }, secure
};

#[tracing::instrument(level="info", skip(conn, _session, config))]
#[post("/", data = "<params>")]
pub async fn action<'a>(_session : CurrentSession, params: JsonIntegrationsParams, conn: Connection<'a, Db>, config: &State<GuilderConfig>) -> CrudResponse<IntegrationModel> {
  let params = params.into_inner();

  if let Err((_, errors)) = env::validate(&params) {
    return CrudResult::fail(errors);
  }

  let mut model = integrations::ActiveModel {
    id: NotSet,
    name: Set(params.name),
    login: Set(params.login.unwrap()),
    kind: Set(params.kind.unwrap()),
    ..Default::default()
  };

  if let Some(password) = params.password {
    let data = secure::encrypt_text(&password, &config.encryption)?;
    model.password = Set(Some(data));
  }

  tracing::debug!("Saving: {:?}", model);

  let resource = model
    .insert(conn.into_inner())
    .await
    .map_err(env::catch_error)?;

  tracing::debug!("Created new integration: {:?}", resource);
  //TODO: trigger sync

  CrudResult::success(resource)
}

#[cfg(test)]
mod test {
  use entity::{Integration, EntityTrait};
  use rocket::{http::{ContentType, Status}, local::asynchronous::{Client, LocalResponse}};
  use crate::test;

  async fn create<'a>(client : &'a Client, body : String) -> LocalResponse<'a> {
    let response = client.post("/api/integrations")
      .header(ContentType::JSON)
      .body(body)
      .dispatch()
      .await;

    return response;
  }

  #[rocket::async_test]
  async fn reject_guest() {
    let (client, _db) = test::server().await;
    let response = create(&client, json!({

    })).await;

    assert_eq!(response.status(), Status::Forbidden);
    assert_body!(response, {
      "errors": vec!["You need to sign in to access: /api/integrations"],
      "status": "forbidden"
    }).await;
  }

  #[rocket::async_test]
  async fn invalid_params() {
    let (client, db) = test::server().await;
    let (_user, _cookie) = test::sign_in(&client, &db).await;

    let response = create(&client, json!({
      "name": "",
      "login": "",
      "password": "",
      "kind": "PKOBP"
    })).await;

    assert_eq!(response.status(), Status::Ok);
    assert_body!(response, {
      "errors": {
        "name": ["minimal length is 1 character"]
      },
      "resource": null,
      "success": false
    }).await;
  }

  #[rocket::async_test]
  async fn test_create_data_source() {
    let (client, db) = test::server().await;
    let (_user, _cookie) = test::sign_in(&client, &db).await;

    let response = create(&client, json!({
      "name": "Pko",
      "login": "1234567",
      "password": "password",
      "kind": "PKOBP"
    })).await;

    assert_eq!(response.status(), Status::Ok);
    assert_body!(response, {
      "errors": null,
      "resource": {
        "id": 1,
        "name": "Pko",
        "status": "Initializing",
        "last_sync_at": null,
        "kind": "PKOBP",
        "login": "1234567"
      },
      "success": true
    }).await;

    let integration = Integration::find_by_id(1).one(&db).await
      .expect("could not fetch data source")
      .expect("missing data source");

    assert_eq!(integration.password, Some(vec![219, 33, 20, 4, 74, 38, 207, 102, 148, 29, 48, 222, 243, 164, 94, 238]));
  }
}
