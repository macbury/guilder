use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
  fn name(&self) -> &str {
    "m20220101_000002_create_users_table"
  }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
  async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    let table = Table::create()
      .table(User::Table)
      .if_not_exists()
      .col(
        ColumnDef::new(User::Id)
        .integer()
        .not_null()
        .auto_increment()
        .primary_key()
      )
      .col(ColumnDef::new(User::Login).string().not_null().unique_key())
      .col(ColumnDef::new(User::HashedPassword).string().not_null())
      .to_owned();
    manager.create_table(table).await
  }

  async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    manager.drop_table(
      Table::drop().table(User::Table).to_owned()
    ).await
  }
}

#[derive(Iden)]
pub enum User {
  Table,
  Id,
  Login,
  HashedPassword
}
