use entity::{wallets, ActiveModelTrait, WalletModel, NotSet};
use sea_orm_rocket::Connection;
use super::types::JsonWalletParams;
use crate::{stages::{Db, session::CurrentSession}, env::{self, CrudResponse, CrudResult}};

#[tracing::instrument(level="info", skip(conn, _session))]
#[post("/", data = "<wallet>")]
pub async fn action<'a>(_session : CurrentSession, wallet: JsonWalletParams, conn: Connection<'a, Db>) -> CrudResponse<WalletModel> {
  let wallet = wallet.into_inner();

  if let Err((_, errors)) = env::validate(&wallet) {
    return CrudResult::fail(errors);
  }

  let mut model = wallets::ActiveModel {
    id: NotSet,
    ..Default::default()
  };

  model.set_from_json(
    serde_json::to_value(wallet)
      .unwrap()
  ).unwrap();

  tracing::debug!("Saving: {:?}", model);

  let resource = model
    .insert(conn.into_inner())
    .await
    .map_err(env::catch_error)?;

  tracing::debug!("Created new wallet: {:?}", resource);

  CrudResult::success(resource)
}

#[cfg(test)]
mod test {
  use rocket::{http::Status, local::asynchronous::{Client, LocalResponse}};
  use crate::test;

  async fn response<'a>(client : &'a Client) -> LocalResponse<'a> {
    let response = client.post(format!("/api/wallets"))
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
      "errors": vec!["You need to sign in to access: /api/wallets"],
      "status": "forbidden"
    }).await;
  }
}
