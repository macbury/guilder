use entity::{bonds, ActiveModelTrait, Set, Bond, BondPerformance, EntityTrait};
use rocket::serde::json::Json;
use sea_orm_rocket::Connection;
use serde::{Deserialize, Serialize};
use validator::Validate;
use crate::{stages::{Db, session::CurrentSession}, env::{self, CrudResponse, CrudResult, catch_error}};
use super::responses::BondResource;

#[derive(Validate, Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BondParams {
  pub update_category: bool,
  pub update_account: bool,
  pub update_wallet: bool,
  pub category_id: Option<i64>,
  pub account_id: Option<i64>,
  pub wallet_id: Option<i64>,
}

pub type JsonBondParams = Json<BondParams>;

#[tracing::instrument(level="info", skip(conn, _session))]
#[put("/<id>", data = "<params>")]
pub async fn action<'a>(_session : CurrentSession, id: i64, params: JsonBondParams, conn: Connection<'a, Db>) -> CrudResponse<BondResource> {
  let db = conn.into_inner();
  let params = params.into_inner();
  let bond = Bond::find_by_id(id)
    .one(db)
    .await
    .map_err(catch_error)?;

  if bond.is_none() {
    return CrudResult::not_found();
  }

  if let Err((_, errors)) = env::validate(&params) {
    return CrudResult::fail(errors);
  }

  let mut model: bonds::ActiveModel = bond.unwrap().into();

  if params.update_category {
    model.category_id = Set(params.category_id);
  }

  if params.update_account {
    model.account_id = Set(params.account_id);
  }

  if params.update_wallet {
    model.wallet_id = Set(params.wallet_id);
  }

  tracing::debug!("Saving: {:?}", model);

  let resource = model
    .update(db)
    .await
    .map_err(env::catch_error)?;

  tracing::debug!("Updated bond: {:?}", resource);

  let resource : Option<BondResource> = Bond::find_by_id(resource.id)
    .find_also_related(BondPerformance)
    .one(db)
    .await
    .map_err(catch_error)?
    .map(|t| t.into());

  if let Some(resource) = resource {
    CrudResult::success(resource)
  } else {
    CrudResult::not_found()
  }
}
