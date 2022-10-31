use crate::view::{create_view, drop_view};
use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
  fn name(&self) -> &str {
    "m20220714_190556_create_accounts_with_balances_view"
  }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
  async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    create_view(manager, include_str!("../sql/views/create_account_balance_view.sql")).await
  }

  async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    drop_view(manager, "accounts_metadata").await
  }
}
