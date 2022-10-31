use sea_orm_migration::prelude::*;

use crate::view::{create_view, drop_materialized_view};

pub struct Migration;

impl MigrationName for Migration {
  fn name(&self) -> &str {
    "m20220630_191253_update_bond_performances"
  }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
  async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    drop_materialized_view(manager, "bond_performances").await?;
    create_view(manager, include_str!("../sql/views/update_view_bond_performances.sql")).await
  }

  async fn down(&self, _manager: &SchemaManager) -> Result<(), DbErr> {
    todo!("You can't revert this...")
  }
}
