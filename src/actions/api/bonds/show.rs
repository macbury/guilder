use entity::{Bond, BondPerformance, EntityTrait};
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

use super::responses::BondResource;

#[derive(Serialize, Debug)]
pub struct ShowResponse {
  bond: BondResource
}

#[get("/<id>")]
pub async fn action<'a>(conn: Connection<'a, Db>, _s : CurrentSession, id : i64) -> ResponseResult<Option<Json<ShowResponse>>> {
  let db = conn.into_inner();
  let bond = Bond::find_by_id(id)
    .find_also_related(BondPerformance)
    .one(db)
    .await
    .map_err(catch_error)?
    .map(|models| models.into());

  let response = bond.map(|bond| Json(ShowResponse { bond: bond }));

  Ok(response)
}

#[cfg(test)]
mod test {
  use rocket::{http::Status, local::asynchronous::{Client, LocalResponse}};
  use crate::test;

  async fn response<'a>(client : &'a Client) -> LocalResponse<'a> {
    let response = client.get(format!("/api/bonds/1"))
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
      "errors": vec!["You need to sign in to access: /api/bonds/1"],
      "status": "forbidden"
    }).await;
  }
}
