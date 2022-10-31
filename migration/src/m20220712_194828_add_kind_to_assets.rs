use entity::Asset;
use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
  fn name(&self) -> &str {
    "m20220712_194828_add_kind_to_assets"
  }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
  async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    let kind_column = Table::alter()
      .table(Asset)
      .add_column(
        ColumnDef::new(Alias::new("kind")).string().default("unknown")
      ).to_owned();

    manager.alter_table(kind_column).await
  }

  async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    let kind_column = Table::alter()
      .table(Asset)
      .drop_column(
        Alias::new("kind")
      ).to_owned();

    manager.alter_table(kind_column).await
  }
}
