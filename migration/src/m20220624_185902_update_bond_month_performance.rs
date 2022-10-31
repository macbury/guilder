use sea_orm_migration::prelude::*;

use crate::view::{create_view, drop_view};

pub struct Migration;

impl MigrationName for Migration {
  fn name(&self) -> &str {
    "m20220624_185902_update_bond_month_performance"
  }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
  async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    drop_view(manager, "bonds_monthly_performance").await?;

    let sql = include_str!("../sql/views/create_bond_month_performance.sql");
    create_view(manager, sql).await
  }

  async fn down(&self, _manager: &SchemaManager) -> Result<(), DbErr> {
    Ok(()) // just leave current view
  }
}
