use entity::Bond;
use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
  fn name(&self) -> &str {
    "m20220621_185134_add_periods_to_bonds"
  }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
  async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    let period_start_column = Table::alter()
      .table(Bond)
      .add_column(
        ColumnDef::new(Alias::new("period_start")).json().default("[]")
      ).to_owned();
    let period_end_column = Table::alter()
      .table(Bond)
      .add_column(
        ColumnDef::new(Alias::new("period_end")).json().default("[]")
      ).to_owned();

    manager.alter_table(period_start_column).await?;
    manager.alter_table(period_end_column).await
  }

  async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    let period_start_column = Table::alter()
      .table(Bond)
      .drop_column(
        Alias::new("period_start")
      ).to_owned();

    let period_end_column = Table::alter()
      .table(Bond)
      .drop_column(
        Alias::new("period_end")
      ).to_owned();

    manager.alter_table(period_start_column).await?;
    manager.alter_table(period_end_column).await?;
    Ok(())
  }
}
