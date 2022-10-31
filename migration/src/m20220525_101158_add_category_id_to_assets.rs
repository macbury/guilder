use entity::{Asset, Category, categories};
use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
  fn name(&self) -> &str {
    "m20220525_101158_add_category_id_to_assets"
  }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
  async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    let add_asset_id = Table::alter()
      .table(Asset)
      .add_column(
        ColumnDef::new(Alias::new("category_id")).big_integer()
      ).to_owned();

    manager.alter_table(add_asset_id).await?;

    manager.create_foreign_key(
      ForeignKey::create()
        .name("category_assets")
        .to(Category, categories::Column::Id)
        .from(Asset, Alias::new("category_id"))
        .on_delete(ForeignKeyAction::SetNull)
        .to_owned()
    ).await?;

    let idx = sea_query::Index::create()
      .name("assets_category_id")
      .table(Asset)
      .col(Alias::new("category_id"))
      .to_owned();

    manager.create_index(idx).await?;

    Ok(())
  }

  async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    let idx = sea_query::Index::drop()
      .name("assets_category_id")
      .table(Asset)
      .to_owned();

    manager.drop_index(idx).await?;

    manager.drop_foreign_key(
      ForeignKey::drop()
        .table(Asset)
        .name("category_assets")
        .to_owned()
    ).await?;

    let remove_asset_id = Table::alter()
      .table(Asset)
      .drop_column(
        Alias::new("category_id")
      ).to_owned();

    manager.alter_table(remove_asset_id).await?;
    Ok(())
  }
}
