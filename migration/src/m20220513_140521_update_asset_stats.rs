use entity::ConnectionTrait;
use sea_orm::Statement;
use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
  fn name(&self) -> &str {
    "m20220513_140521_update_asset_stats"
  }
}


#[async_trait::async_trait]
impl MigrationTrait for Migration {
  async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    self.down(manager).await?;
    let sql = include_str!("../sql/update_view_asset_stats.sql");
    let stmt = Statement::from_string(manager.get_database_backend(), sql.to_owned());
    manager.get_connection().execute(stmt).await.map(|_| ())
  }

  async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    let sql = "DROP VIEW asset_performances;";
    let stmt = Statement::from_string(manager.get_database_backend(), sql.to_owned());
    manager.get_connection().execute(stmt).await.map(|_| ())
  }
}
