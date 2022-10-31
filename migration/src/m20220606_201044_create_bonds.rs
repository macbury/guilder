use entity::{Category, categories, Account, accounts, Integration, integrations};
use sea_orm::Schema;
use sea_orm_migration::prelude::*;

pub mod bonds {
  use sea_orm::entity::prelude::*;

  #[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
  #[sea_orm(table_name = "bonds")]
  pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i64,
    pub name: String,
    #[sea_orm(unique = true)]
    pub uid: String,
    pub emission: String,
    pub status : String,
    pub kind : String,
    pub currency : String,
    pub category_id: Option<i64>,
    pub integration_id: Option<i64>,
    pub account_id: Option<i64>,
    pub start_date: Date,
    pub end_date: Date,
    pub interest_date: Date,
    pub updated_at: DateTime,
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
    "m20220606_201044_create_bonds"
  }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
  async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    let builder = manager.get_database_backend();
    let schema = Schema::new(builder);

    manager
      .create_table(schema.create_table_from_entity(bonds::Entity))
      .await?;

    manager.create_foreign_key(
    ForeignKey::create()
      .name("category_bonds")
      .from(bonds::Entity, bonds::Column::CategoryId)
      .to(Category, categories::Column::Id)
      .on_delete(ForeignKeyAction::SetNull)
      .to_owned()
    ).await?;

    manager.create_foreign_key(
      ForeignKey::create()
        .name("account_bonds")
        .from(bonds::Entity, bonds::Column::AccountId)
        .to(Account, accounts::Column::Id)
        .on_delete(ForeignKeyAction::SetNull)
        .to_owned()
      ).await?;


    manager.create_foreign_key(
      ForeignKey::create()
        .name("integration_bonds")
        .from(bonds::Entity, bonds::Column::IntegrationId)
        .to(Integration, integrations::Column::Id)
        .on_delete(ForeignKeyAction::Cascade)
        .to_owned()
      ).await?;

    Ok(())
  }

  async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    manager
      .drop_table(Table::drop().table(bonds::Entity).to_owned())
      .await
  }
}
