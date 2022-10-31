use sea_orm::Schema;
use sea_orm_migration::prelude::*;

pub mod integrations {
  use sea_orm::entity::prelude::*;

  #[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
  #[sea_orm(table_name = "integrations")]
  pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i64,
    pub name: String,
    #[sea_orm(default_value="Initializing")]
    pub status: String,
    pub last_sync_at: Option<DateTimeUtc>,
    pub kind: String,
    pub login: String,
    pub password: Option<Vec<u8>>,
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
    "m20220603_140834_create_data_sources"
  }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
  async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    let builder = manager.get_database_backend();
    let schema = Schema::new(builder);

    manager
      .create_table(schema.create_table_from_entity(integrations::Entity))
      .await
  }

  async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    manager
      .drop_table(Table::drop().table(integrations::Entity).to_owned())
      .await
  }
}
