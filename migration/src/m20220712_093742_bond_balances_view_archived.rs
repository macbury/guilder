use sea_orm_migration::prelude::*;
use crate::view::{create_view, drop_view};

pub struct Migration;

impl MigrationName for Migration {
  fn name(&self) -> &str {
    "m20220712_093742_bond_balances_view_archived"
  }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
  async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    drop_view(manager, "bond_balance_performances").await?;
    create_view(manager, include_str!("../sql/views/update_bond_balance_performances.sql")).await
  }

  async fn down(&self, _manager: &SchemaManager) -> Result<(), DbErr> {
    todo!()
  }
}
