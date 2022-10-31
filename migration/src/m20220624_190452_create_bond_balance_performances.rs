use sea_orm_migration::prelude::*;
use crate::view::{create_view, drop_view};

pub struct Migration;

impl MigrationName for Migration {
  fn name(&self) -> &str {
    "m20220624_190452_create_bond_balance_performances"
  }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
  async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    let sql = include_str!("../sql/views/create_bond_balance_performances.sql");
    create_view(manager, sql).await
  }

  async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    drop_view(manager, "bond_balance_performances").await
  }
}
