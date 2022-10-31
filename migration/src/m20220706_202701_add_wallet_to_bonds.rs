use entity::{Bond, Wallet, wallets};
use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
  fn name(&self) -> &str {
    "m20220706_202701_add_wallet_to_bonds"
  }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
  async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    let add_wallet_id = Table::alter()
      .table(Bond)
      .add_column(
        ColumnDef::new(Alias::new("wallet_id")).big_integer()
      ).to_owned();

    manager.alter_table(add_wallet_id).await?;

    manager.create_foreign_key(
      ForeignKey::create()
        .name("wallet_bonds")
        .to(Wallet, wallets::Column::Id)
        .from(Bond, Alias::new("wallet_id"))
        .on_delete(ForeignKeyAction::SetNull)
        .to_owned()
    ).await?;

    let idx = sea_query::Index::create()
      .name("wallet_bonds_ids")
      .table(Bond)
      .col(Alias::new("wallet_id"))
      .to_owned();

    manager.create_index(idx).await?;

    Ok(())
  }

  async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    let idx = sea_query::Index::drop()
      .name("wallet_bonds_ids")
      .table(Wallet)
      .to_owned();

    manager.drop_index(idx).await?;

    manager.drop_foreign_key(
      ForeignKey::drop()
        .table(Bond)
        .name("wallet_bonds")
        .to_owned()
    ).await?;

    let remove_wallet_id = Table::alter()
      .table(Bond)
      .drop_column(
        Alias::new("wallet_id")
      ).to_owned();

    manager.alter_table(remove_wallet_id).await?;
    Ok(())
  }
}
