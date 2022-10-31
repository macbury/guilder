use rocket::serde::json::Json;
use serde::Serialize;
use sea_orm_rocket::Connection;
use super::responses::BondResource;
use super::sort::{SortBy, SortDirection, Scope};

use entity::{
  bonds,
  Condition,
  BondPerformance,
  EntityTrait,
  Bond,
  Expr,
  QueryFilter
};

use crate::{
  stages::{session::CurrentSession, Db},
  env::{ResponseResult, catch_error}
};

#[derive(Serialize, Debug)]
pub struct ListAllResourcesResponse {
  bonds: Vec<BondResource>,
}

type ModelIds = Option<Vec<i64>>;

#[tracing::instrument(name="all_bonds", level="info", skip(conn, _session))]
#[get("/?<sort>&<direction>&<name>&<accounts>&<wallets>&<categories>&<scope>")]
pub async fn action<'a>(conn: Connection<'a, Db>, _session : CurrentSession, accounts: ModelIds, wallets: ModelIds, categories: ModelIds, name: Option<&'a str>, scope: Option<Scope>, sort: Option<SortBy>, direction: Option<SortDirection>) -> ResponseResult<Json<ListAllResourcesResponse>> {
  let sort_column = sort.unwrap_or_default();
  let db = conn.into_inner();
  let query = Bond::find()
    .find_also_related(BondPerformance);
  let query = sort_column.apply(query, direction);
  let query = scope.unwrap_or_default().apply(query);

  let name_match = name.map(|name| {
    Condition::any()
      .add(Expr::col(bonds::Column::Name).matches(Expr::val(name)))
      .add(Expr::col(bonds::Column::Emission).matches(Expr::val(name)))
  });

  let conditions = Condition::all()
    .add_option(name_match)
    .add_option(accounts.map(|accs| Expr::col(bonds::Column::AccountId).is_in(accs)))
    .add_option(wallets.map(|wids| Expr::col(bonds::Column::WalletId).is_in(wids)))
    .add_option(categories.map(|cats| Expr::col(bonds::Column::CategoryId).is_in(cats)));

  let bonds : Vec<BondResource> = query
    .filter(conditions)
    .all(db)
    .await
    .map_err(catch_error)?
    .iter()
    .map(|models| models.into())
    .collect();

  tracing::info!("Found: {:?} bonds", bonds.len());

  Ok(Json(ListAllResourcesResponse { bonds }))
}

#[cfg(test)]
mod test {
  use rocket::{http::Status, local::asynchronous::{Client, LocalResponse}};
  use crate::test;

  async fn response<'a>(client : &'a Client) -> LocalResponse<'a> {
    let response = client.get(format!("/api/bonds"))
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
      "errors": vec!["You need to sign in to access: /api/bonds"],
      "status": "forbidden"
    }).await;
  }
}
