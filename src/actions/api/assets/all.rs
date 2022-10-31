use entity::{
  Condition,
  AssetPerformance,
  EntityTrait,
  Asset,
  assets, Expr, QueryFilter,
};
use rocket::serde::json::Json;
use serde::Serialize;
use sea_orm_rocket::Connection;
use super::sort::{SortBy, SortDirection};

use crate::{
  stages::{
    session::CurrentSession,
    Db
  },
  env::{ResponseResult, catch_error},
  types::AssetDetails
};

#[derive(Serialize, Debug)]
pub struct ShowAssetsResponse {
  assets: Vec<AssetDetails>,
}

#[tracing::instrument(name="all_assets", level="info", skip(conn, _session))]
#[get("/?<sort>&<direction>&<name>&<categories>")]
pub async fn action<'a>(conn: Connection<'a, Db>, _session : CurrentSession, categories: Option<Vec<i64>>, name: Option<&'a str>, sort: Option<SortBy>, direction: Option<SortDirection>) -> ResponseResult<Json<ShowAssetsResponse>> {
  let sort_column = sort.unwrap_or_default();
  let db = conn.into_inner();
  let query = Asset::find()
    .find_also_related(AssetPerformance);
  let query = sort_column.apply(query, direction);

  let name_match = name.map(|name| {
    Condition::any()
      .add(Expr::col(assets::Column::Name).matches(Expr::val(name)))
      .add(Expr::col(assets::Column::Id).matches(Expr::val(name)))
  });

  let conditions = Condition::all()
    .add_option(name_match)
    .add_option(categories.map(|cats| Expr::col(assets::Column::CategoryId).is_in(cats)));

  let assets : Vec<AssetDetails> = query
    .filter(conditions)
    .all(db)
    .await
    .map_err(catch_error)?
    .iter()
    .map(|asset| asset.into())
    .collect();

  tracing::info!("Found: {:?} assets", assets.len());

  Ok(Json(ShowAssetsResponse { assets }))
}
