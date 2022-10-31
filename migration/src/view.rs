use entity::{Statement, ConnectionTrait};
use sea_orm_migration::prelude::*;

pub async fn create_view<'a>(manager: &'a SchemaManager<'a>, sql: &'a str) -> Result<(), DbErr> {
  let stmt = Statement::from_string(manager.get_database_backend(), sql.to_owned());
  manager.get_connection().execute(stmt).await.map(|_| ())
}

pub async fn drop_view<'a>(manager: &'a SchemaManager<'a>, name: &'a str) -> Result<(), DbErr> {
  let sql = format!("DROP VIEW IF EXISTS {} CASCADE;", name);
  let stmt = Statement::from_string(manager.get_database_backend(), sql.to_owned());
  manager.get_connection().execute(stmt).await.map(|_| ())
}

pub async fn drop_materialized_view<'a>(manager: &'a SchemaManager<'a>, name: &'a str) -> Result<(), DbErr> {
  let sql = format!("DROP MATERIALIZED VIEW IF EXISTS {};", name);
  let stmt = Statement::from_string(manager.get_database_backend(), sql.to_owned());
  manager.get_connection().execute(stmt).await.map(|_| ())
}
