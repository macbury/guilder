use sea_orm::Schema;
use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
  fn name(&self) -> &str {
    "m20220704_183106_create_wallets"
  }
}

pub mod wallets {
  use sea_orm::entity::prelude::*;

  #[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
  #[sea_orm(table_name = "wallets")]
  pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i64,
    pub name: String,
    pub currency : String,
    pub description : String,
  }

  #[derive(Copy, Clone, Debug, EnumIter)]
  pub enum Relation {
  }

  impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
      todo!()
    }
  }

  impl ActiveModelBehavior for ActiveModel {}
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
  async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    let builder = manager.get_database_backend();
    let schema = Schema::new(builder);

    manager
      .create_table(schema.create_table_from_entity(wallets::Entity))
      .await?;

    Ok(())
  }

  async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    manager
      .drop_table(Table::drop().table(wallets::Entity).to_owned())
      .await
  }
}
