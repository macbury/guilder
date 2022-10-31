use entity::{InterestRateHistory, EntityTrait, QueryOrder, interest_rate_histories, Bond, bond_period::BondPeriods, BondPerformance};
use rocket::{serde::json::Json};
use serde::Serialize;
use anyhow::{anyhow, Context};
use sea_orm_rocket::Connection;

use crate::{
  stages::{
    session::CurrentSession,
    Db
  },
  env::{ResponseResult, catch_error},
};

#[derive(Serialize, Debug)]
pub struct HistoryResponse {
  periods: BondPeriods,
  history: Vec<interest_rate_histories::Model>,
}

#[get("/<id>/interest_rates")]
pub async fn action<'a>(conn: Connection<'a, Db>, _s : CurrentSession, id : i64) -> ResponseResult<Json<HistoryResponse>> {
  let db = conn.into_inner();
  let bond = Bond::find_by_id(id)
    .find_with_related(BondPerformance)
    .one(db)
    .await
    .map_err(catch_error)?
    .ok_or(anyhow!("Missing bond!"))?;

  let history = InterestRateHistory::find_by_id(id)
    .order_by_asc(interest_rate_histories::Column::Date)
    .all(db)
    .await
    .map_err(catch_error)?;

  tracing::info!("Found {:?} history entries!", history.len());
  let periods : BondPeriods = bond.try_into()
    .with_context(|| "could not convert bond with performance into periods...")?;

  Ok(
    Json(
      HistoryResponse {
        history,
        periods
      }
    )
  )
}

#[cfg(test)]
mod test {
  use rocket::{http::Status, local::asynchronous::{Client, LocalResponse}};
  use crate::test;

  async fn response<'a>(client : &'a Client) -> LocalResponse<'a> {
    let response = client.get(format!("/api/bonds/1/interest_rates"))
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
      "errors": vec!["You need to sign in to access: /api/bonds/1/interest_rates"],
      "status": "forbidden"
    }).await;
  }
}
