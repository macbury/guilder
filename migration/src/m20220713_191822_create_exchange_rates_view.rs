use crate::view::{create_view, drop_materialized_view};
use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
  fn name(&self) -> &str {
    "m20220713_191822_create_exchange_rates_view"
  }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
  async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    drop_materialized_view(manager, "exchange_rates").await?;
    create_view(manager, include_str!("../sql/views/create_exchange_rates.sql")).await
  }

  async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    drop_materialized_view(manager, "exchange_rates").await
  }
}
