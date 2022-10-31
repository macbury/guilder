use entity::Statement;
use entity::ConnectionTrait;
use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
  fn name(&self) -> &str {
    "m20220608_185348_create_fk_bonds_points"
  }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
  async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    let sql = include_str!("../sql/create_points_bond.sql");
    let stmt = Statement::from_string(manager.get_database_backend(), sql.to_owned());
    manager.get_connection().execute(stmt).await.map(|_| ())?;

    let sql = "CREATE TRIGGER drop_points_after_bond_delete BEFORE DELETE ON bonds FOR EACH ROW EXECUTE PROCEDURE drop_points_after_bond();";
    let stmt = Statement::from_string(manager.get_database_backend(), sql.to_owned());
    manager.get_connection().execute(stmt).await.map(|_| ())
  }

  async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    let sql = "DROP TRIGGER IF EXISTS drop_points_after_bond_delete ON bonds;";
    let stmt = Statement::from_string(manager.get_database_backend(), sql.to_owned());
    manager.get_connection().execute(stmt).await.map(|_| ())?;

    let sql = "DROP FUNCTION drop_points_after_bond;";
    let stmt = Statement::from_string(manager.get_database_backend(), sql.to_owned());
    manager.get_connection().execute(stmt).await.map(|_| ())
  }
}
