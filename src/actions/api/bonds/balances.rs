use entity::{BondBalancePerformanceModel, BondBalancePerformance, EntityTrait, QueryOrder, bond_balance_performances};
use rocket::{serde::json::Json};
use serde::Serialize;
use sea_orm_rocket::Connection;

use crate::{
  stages::{
    session::CurrentSession,
    Db
  },
  env::{ResponseResult, catch_error},
};

#[derive(Serialize, Debug)]
pub struct BondsPerformance {
  performance: Vec<BondBalancePerformanceModel>,
}

#[get("/balances")]
pub async fn action<'a>(conn: Connection<'a, Db>, _s : CurrentSession) -> ResponseResult<Json<BondsPerformance>> {
  let db = conn.into_inner();

  let performance = BondBalancePerformance::find()
    .order_by_asc(bond_balance_performances::Column::Month)
    .all(db)
    .await
    .map_err(catch_error)?;

  Ok(
    Json(
      BondsPerformance {
        performance
      }
    )
  )
}

#[cfg(test)]
mod test {
  use rocket::{http::Status, local::asynchronous::{Client, LocalResponse}};
  use crate::test;

  async fn response<'a>(client : &'a Client) -> LocalResponse<'a> {
    let response = client.get("/api/bonds/balances")
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
      "errors": vec!["You need to sign in to access: /api/bonds/balances"],
      "status": "forbidden"
    }).await;
  }
}
