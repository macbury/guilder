use entity::{assets, ActiveModelTrait, Set, Asset, AssetPerformance, EntityTrait};
use rocket::serde::json::Json;
use sea_orm_rocket::Connection;
use serde::{Deserialize, Serialize};
use trading_view::Ticker;
use validator::Validate;
use crate::{stages::{Db, session::CurrentSession, trading_view::TickerParam}, env::{self, CrudResponse, CrudResult, catch_error}, types::AssetDetails};

#[derive(Validate, Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AssetParams {
  pub category_id: Option<i64>,
}

pub type JsonAssetParams = Json<AssetParams>;

#[tracing::instrument(level="info", skip(conn, _session))]
#[put("/<ticker>", data = "<params>")]
pub async fn action<'a>(_session : CurrentSession, ticker: TickerParam<'a>, params: JsonAssetParams, conn: Connection<'a, Db>) -> CrudResponse<AssetDetails> {
  let ticker : Ticker = ticker.into();

  let db = conn.into_inner();
  let params = params.into_inner();
  let asset = Asset::find_by_ticker(&ticker)
    .one(db)
    .await
    .map_err(catch_error)?;

  if asset.is_none() {
    return CrudResult::not_found();
  }

  if let Err((_, errors)) = env::validate(&params) {
    return CrudResult::fail(errors);
  }

  let mut model: assets::ActiveModel = asset.unwrap().into();
  model.category_id = Set(params.category_id);

  tracing::debug!("Saving: {:?}", model);

  let resource = model
    .update(db)
    .await
    .map_err(env::catch_error)?;

  tracing::debug!("Updated asset: {:?}", resource);

  let resource : Option<AssetDetails> = Asset::find_by_id(resource.id)
    .find_also_related(AssetPerformance)
    .one(db)
    .await
    .map_err(catch_error)?
    .map(|a| a.into());


  if let Some(resource) = resource {
    CrudResult::success(resource)
  } else {
    CrudResult::not_found()
  }

}

//TODO: tests
