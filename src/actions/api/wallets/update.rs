use entity::{wallets, ActiveModelTrait, WalletModel, Wallet, EntityTrait};
use sea_orm_rocket::Connection;
use super::types::JsonWalletParams;
use crate::{stages::{Db, session::CurrentSession}, env::{self, CrudResponse, CrudResult}};

#[tracing::instrument(level="info", skip(conn, _session))]
#[put("/<wallet_id>", data = "<params>")]
pub async fn action<'a>(_session : CurrentSession, wallet_id: i64, params: JsonWalletParams, conn: Connection<'a, Db>) -> CrudResponse<WalletModel> {
  let db = conn.into_inner();
  let params = params.into_inner();
  let wallet = Wallet::find_by_id(wallet_id)
    .one(db)
    .await
    .map_err(env::catch_error)?;

  if wallet.is_none() {
    return CrudResult::not_found();
  }

  if let Err((_, errors)) = env::validate(&params) {
    return CrudResult::fail(errors);
  }

  let mut model: wallets::ActiveModel = wallet.unwrap().into();
  model.set_from_json(
    serde_json::to_value(params)
      .unwrap()
  ).unwrap();

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
    let response = client.put(format!("/api/wallets/1"))
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
      "errors": vec!["You need to sign in to access: /api/wallets/1"],
      "status": "forbidden"
    }).await;
  }
}
