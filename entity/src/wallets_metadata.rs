//! SeaORM Entity. Generated by sea-orm-codegen 0.8.0

use sea_orm::entity::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "wallets_metadata")]
pub struct Model {
  #[sea_orm(primary_key)]
  pub wallet_id: i64,
  pub balance: f64
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
  #[sea_orm(
    belongs_to = "super::wallets::Entity",
    from = "Column::WalletId",
    to = "super::wallets::Column::Id",
    on_update = "NoAction",
    on_delete = "NoAction"
  )]
  Wallet
}

impl Related<super::wallets::Entity> for Entity {
  fn to() -> RelationDef {
    Relation::Wallet.def()
  }
}

impl ActiveModelBehavior for ActiveModel {}