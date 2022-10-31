use sea_orm::Schema;
use sea_orm_migration::prelude::*;

pub mod accounts {
  use sea_orm::entity::prelude::*;

  #[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
  #[sea_orm(table_name = "accounts")]
  pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i64,
    pub name: String,
    pub description: String,
    pub currency: String
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

pub struct Migration;

impl MigrationName for Migration {
  fn name(&self) -> &str {
    "m20220530_102824_create_accounts"
  }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
  async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    let builder = manager.get_database_backend();
    let schema = Schema::new(builder);

    manager
      .create_table(schema.create_table_from_entity(accounts::Entity))
      .await
  }

  async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    manager
      .drop_table(Table::drop().table(accounts::Entity).to_owned())
      .await
  }
}
