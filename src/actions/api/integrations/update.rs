use entity::{integrations, ActiveModelTrait, Set, EntityTrait, Integration, IntegrationModel};
use rocket::State;
use sea_orm_rocket::Connection;
use super::types::JsonIntegrationsParams;
use crate::{
  stages::{Db, session::CurrentSession},
  env::{self, CrudResponse, CrudResult, GuilderConfig},
  secure,
};

#[tracing::instrument(level="info", skip(conn, config, _session))]
#[put("/<id>", data = "<params>")]
pub async fn action<'a>(_session : CurrentSession, id: i64, params: JsonIntegrationsParams, conn: Connection<'a, Db>, config: &State<GuilderConfig>) -> CrudResponse<IntegrationModel> {
  let db = conn.into_inner();
  let params = params.into_inner();
  let integration = Integration::find_by_id(id)
    .one(db)
    .await
    .map_err(env::catch_error)?;

  if integration.is_none() {
    return CrudResult::not_found();
  }

  if let Err((_, errors)) = env::validate(&params) {
    return CrudResult::fail(errors);
  }

  let mut model: integrations::ActiveModel = integration.unwrap().into();
  model.name = Set(params.name);
  model.login = Set(params.login.unwrap());

  if let Some(password) = params.password {
    let data = secure::encrypt_text(&password, &config.encryption)?;
    model.password = Set(Some(data));
  }

  tracing::debug!("Saving: {:?}", model);

  let resource = model
    .update(db)
    .await
    .map_err(env::catch_error)?;

  tracing::debug!("updated resource: {:?}", resource);

  CrudResult::success(resource)
}

#[cfg(test)]
mod test {
  use rocket::{http::Status, local::asynchronous::{Client, LocalResponse}};
  use crate::test;

  async fn request<'a>(client : &'a Client, id: i64) -> LocalResponse<'a> {
    let response = client.put(format!("/api/integrations/{}", id))
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
