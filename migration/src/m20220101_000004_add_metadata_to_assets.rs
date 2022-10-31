use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
  fn name(&self) -> &str {
    "m20220101_000004_add_metadata_to_assets"
  }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
  async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    manager
      .alter_table(Table::alter()
      .table(entity::assets::Entity)
        .add_column(ColumnDef::new(Alias::new("country")).string())
      .to_owned()).await?;

    manager
      .alter_table(Table::alter()
      .table(entity::assets::Entity)
        .add_column(ColumnDef::new(Alias::new("currency_logo_url")).string())
      .to_owned()).await?;

    manager
      .alter_table(Table::alter()
      .table(entity::assets::Entity)
        .add_column(ColumnDef::new(Alias::new("exchange")).string())
      .to_owned()).await
  }

  async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    let table = Table::alter()
      .table(entity::assets::Entity)
        .drop_column(Alias::new("country"))
        .drop_column(Alias::new("currency_logo_url"))
        .drop_column(Alias::new("exchange"))
      .to_owned();
    manager.alter_table(table).await
  }
}
