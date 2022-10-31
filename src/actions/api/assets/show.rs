use entity::{Asset, AssetPerformance};
use rocket::{serde::json::Json, State};
use serde::Serialize;
use sea_orm_rocket::Connection;
use trading_view::{AssetsManager, Ticker};

use crate::{
  stages::{
    session::CurrentSession,
    trading_view::TickerParam,
    Db
  },
  env::{ResponseResult, catch_error},
  types::AssetDetails
};

#[derive(Serialize, Debug)]
pub struct ShowAssetResponse {
  asset: Option<AssetDetails>
}

#[get("/<ticker>")]
pub async fn action<'a>(conn: Connection<'a, Db>, _s : CurrentSession, ticker : TickerParam<'a>, am : &State<AssetsManager>) -> ResponseResult<Json<ShowAssetResponse>> {
  let ticker : Ticker = ticker.into();

  let db = conn.into_inner();
  let asset = Asset::find_by_ticker(&ticker)
    .find_also_related(AssetPerformance)
    .one(db)
    .await
    .map_err(catch_error)?;

  if let Some(asset) = asset {
    Ok(Json(ShowAssetResponse { asset: Some(asset.into()) }))
  } else {
    let mut am = am.inner().clone();
    am.observe(ticker).await?;
    Ok(Json(ShowAssetResponse { asset: None }))
  }
}
