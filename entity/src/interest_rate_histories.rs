//! SeaORM Entity. Generated by sea-orm-codegen 0.7.0

use sea_orm::entity::prelude::*;
use sea_orm::{EntityTrait};
use serde::Serialize;

#[derive(Serialize, Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "interest_rates_history")]
#[serde(rename_all = "camelCase")]
pub struct Model {
  #[sea_orm(primary_key, auto_increment = false)]
  pub bond_id: i64,
  pub period: i64,
  pub currency: String,
  pub date: Date,
  pub start_price: f64,
  pub price: f64,
  pub rate: f64,
  pub last_price: f64,
  pub price_change: f64,
  pub percent_change: f64,
  pub day_price_change: f64,
  pub day_percent_change: f64
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
  Bond
}

impl RelationTrait for Relation {
  fn def(&self) -> RelationDef {
    Entity::belongs_to(super::bonds::Entity)
      .from(Column::BondId)
      .to(super::bonds::Column::Id)
      .into()
  }
}

impl Related<super::bonds::Entity> for Entity {
  fn to() -> RelationDef {
    Relation::Bond.def()
  }
}

impl ActiveModelBehavior for ActiveModel {}

impl From<&Model> for Model {
  fn from(model: &Model) -> Self {
    let model = model.clone();
    model.into()
  }
}
