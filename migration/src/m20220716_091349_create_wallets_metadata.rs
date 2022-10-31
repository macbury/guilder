use crate::view::{create_view, drop_view};
use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
  fn name(&self) -> &str {
    "m20220716_091349_create_wallets_metadata"
  }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
  async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    create_view(manager, include_str!("../sql/views/create_wallet_balance_view.sql")).await
  }

  async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    drop_view(manager, "wallets_metadata").await
  }
}
