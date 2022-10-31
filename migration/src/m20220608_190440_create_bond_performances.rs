use entity::{Statement, ConnectionTrait};
use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
  fn name(&self) -> &str {
    "m20220608_190440_create_bond_performances"
  }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
  async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    let sql = include_str!("../sql/create_view_bond_performances.sql");
    let stmt = Statement::from_string(manager.get_database_backend(), sql.to_owned());
    manager.get_connection().execute(stmt).await.map(|_| ())
  }

  async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    let sql = "DROP MATERIALIZED VIEW IF EXISTS bond_performances;";
    let stmt = Statement::from_string(manager.get_database_backend(), sql.to_owned());
    manager.get_connection().execute(stmt).await.map(|_| ())
  }
}
