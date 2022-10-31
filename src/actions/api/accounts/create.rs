use entity::{accounts, ActiveModelTrait, AccountModel, ActiveValue::NotSet, Set};
use rocket::State;
use sea_orm_rocket::Connection;
use super::types::JsonAccountParams;
use crate::{stages::{Db, session::CurrentSession, exchange_rate::StateExchangeRates}, env::{self, CrudResponse, CrudResult}};

#[tracing::instrument(level="info", skip(conn, _session, exchange_rates))]
#[post("/", data = "<account>")]
pub async fn action<'a>(_session : CurrentSession, account: JsonAccountParams, conn: Connection<'a, Db>, exchange_rates : &State<StateExchangeRates>) -> CrudResponse<AccountModel> {
  let account = account.into_inner();
  if let Err((_, errors)) = env::validate(&account) {
    return CrudResult::fail(errors);
  }

  let model = accounts::ActiveModel {
    id: NotSet,
    name: Set(account.name),
    description: Set(account.description),
    currency: Set(account.currency),
    ..Default::default()
  };

  tracing::debug!("Saving: {:?}", model);

  let resource = model
    .insert(conn.into_inner())
    .await
    .map_err(env::catch_error)?;

  tracing::debug!("Created new account: {:?}", resource);
  let mut exchange_rates = exchange_rates.write().await;
  exchange_rates.sync().await?;

  CrudResult::success(resource)
}

#[cfg(test)]
mod test {
  use rocket::{http::Status, local::asynchronous::{Client, LocalResponse}};
  use crate::test;

  async fn response<'a>(client : &'a Client) -> LocalResponse<'a> {
    let response = client.post(format!("/api/accounts"))
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
      "errors": vec!["You need to sign in to access: /api/accounts"],
      "status": "forbidden"
    }).await;
  }
}
