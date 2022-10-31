use sea_orm_migration::prelude::*;
use crate::view::{create_view, drop_view};
pub struct Migration;

impl MigrationName for Migration {
  fn name(&self) -> &str {
    "m20220630_194402_update_bond_balance_performances"
  }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
  async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    drop_view(manager, "interest_rates_history").await?;
    create_view(manager, include_str!("../sql/views/update_interest_rates.sql")).await?;
    create_view(manager, include_str!("../sql/views/update_bond_month_performance.sql")).await?;
    create_view(manager, include_str!("../sql/views/update_bond_balance_performances.sql")).await
  }

  async fn down(&self, _manager: &SchemaManager) -> Result<(), DbErr> {
    Ok(())
  }
}
