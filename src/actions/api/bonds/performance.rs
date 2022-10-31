use entity::{BondMonthlyPerformance, BondMonthlyPerformanceModel, bond_monthly_performances, EntityTrait, QueryOrder};
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
pub struct PerformanceResponse {
  performance: Vec<BondMonthlyPerformanceModel>,
}

#[get("/<id>/performance")]
pub async fn action<'a>(conn: Connection<'a, Db>, _s : CurrentSession, id : i64) -> ResponseResult<Json<PerformanceResponse>> {
  let db = conn.into_inner();

  let performance = BondMonthlyPerformance::find_by_id(id)
    .order_by_asc(bond_monthly_performances::Column::Date)
    .all(db)
    .await
    .map_err(catch_error)?;

  Ok(
    Json(
      PerformanceResponse {
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
    let response = client.get(format!("/api/bonds/1/performance"))
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
      "errors": vec!["You need to sign in to access: /api/bonds/1/performance"],
      "status": "forbidden"
    }).await;
  }
}
