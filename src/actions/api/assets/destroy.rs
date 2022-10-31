use entity::{Asset, ModelTrait};
use rocket::{serde::{Serialize, json::{Json}}, State};
use sea_orm_rocket::Connection;
use trading_view::{Ticker, AssetsManager};
use crate::{stages::{Db, session::CurrentSession, trading_view::TickerParam}, env::{ResponseResult, self}};

#[derive(Serialize, Debug)]
pub struct DestroyAssetResponse {
  deleted: u64
}

#[tracing::instrument(level="info", skip(am, conn, _session))]
#[delete("/<ticker>")]
pub async fn action<'a>(_session : CurrentSession, ticker : TickerParam<'a>, conn: Connection<'a, Db>, am : &'a State<AssetsManager>) -> ResponseResult<Option<Json<DestroyAssetResponse>>> {
  let ticker : Ticker = ticker.into();
  let db = conn.into_inner();
  let asset = Asset::find_by_ticker(&ticker)
    .one(db)
    .await
    .map_err(env::catch_error)?;

  if asset.is_none() {
    tracing::info!("Asset is not found");
    return Ok(None)
  }

  let asset = asset.unwrap();
  let result = asset.delete(db).await
    .map_err(env::catch_error)?;
  tracing::info!("Deleted rows: {:?}", result);

  let mut am = am.inner().clone();
    am.forget(ticker).await?;

  Ok(Some(Json(DestroyAssetResponse { deleted: result.rows_affected })))
}
