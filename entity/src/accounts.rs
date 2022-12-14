//! SeaORM Entity. Generated by sea-orm-codegen 0.8.0

use sea_orm::entity::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "accounts")]
pub struct Model {
  #[sea_orm(primary_key)]
  pub id: i64,
  pub name: String,
  pub description: String,
  pub currency: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
  #[sea_orm(
    has_many = "super::accounts_metadata::Entity",
    from = "Column::Id",
    to = "super::accounts_metadata::Column::AccountId",
    on_update = "NoAction",
    on_delete = "NoAction"
  )]
  AccountsMetadata,
}

impl Related<super::accounts_metadata::Entity> for Entity {
  fn to() -> RelationDef {
    Relation::AccountsMetadata.def()
  }
}

impl ActiveModelBehavior for ActiveModel {}
