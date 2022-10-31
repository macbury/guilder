use entity::{accounts, ActiveModelTrait, AccountModel, Set, Account, EntityTrait};
use sea_orm_rocket::Connection;
use super::types::JsonAccountParams;
use crate::{stages::{Db, session::CurrentSession}, env::{self, CrudResponse, CrudResult}};

#[tracing::instrument(level="info", skip(conn, _session))]
#[put("/<account_id>", data = "<params>")]
pub async fn action<'a>(_session : CurrentSession, account_id: i64, params: JsonAccountParams, conn: Connection<'a, Db>) -> CrudResponse<AccountModel> {
  let db = conn.into_inner();
  let params = params.into_inner();
  let account = Account::find_by_id(account_id)
    .one(db)
    .await
    .map_err(env::catch_error)?;

  if account.is_none() {
    return CrudResult::not_found();
  }

  if let Err((_, errors)) = env::validate(&params) {
    return CrudResult::fail(errors);
  }

  let mut model: accounts::ActiveModel = account.unwrap().into();
  model.name = Set(params.name);
  model.description = Set(params.description);
  model.currency = Set(params.currency);

  tracing::debug!("Saving: {:?}", model);

  let resource = model
    .update(db)
    .await
    .map_err(env::catch_error)?;

  tracing::debug!("Created new account: {:?}", resource);

  CrudResult::success(resource)
}

#[cfg(test)]
mod test {
  use rocket::{http::Status, local::asynchronous::{Client, LocalResponse}};
  use crate::test;

  async fn response<'a>(client : &'a Client) -> LocalResponse<'a> {
    let response = client.put(format!("/api/accounts/1"))
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
