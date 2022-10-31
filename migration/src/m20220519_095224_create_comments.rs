use sea_orm::Schema;
use sea_orm_migration::prelude::*;

pub mod comments {
  use sea_orm::entity::prelude::*;

  #[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
  #[sea_orm(table_name = "comments")]
  pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i64,
    pub date: DateTime,
    pub body: String,
    pub user_id: i32,
    pub model_id: String,
    pub model_type: String
  }

  #[derive(Copy, Clone, Debug, EnumIter)]
  pub enum Relation {
    Asset,
    User
  }

  impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
      match self {
        Relation::Asset => {
          Entity::belongs_to(entity::assets::Entity)
            .from(Column::ModelId)
            .to(entity::assets::Column::Id)
            .on_delete(ForeignKeyAction::Cascade)
            .into()
        },

        Relation::User => {
          Entity::belongs_to(entity::user::Entity)
            .from(Column::UserId)
            .to(entity::user::Column::Id)
            .on_delete(ForeignKeyAction::Cascade)
            .into()
        }
      }
    }
  }

  impl ActiveModelBehavior for ActiveModel {}
}

pub struct Migration;

impl MigrationName for Migration {
  fn name(&self) -> &str {
    "m20220519_095224_create_comments"
  }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
  async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    let builder = manager.get_database_backend();
    let schema = Schema::new(builder);

    manager
      .create_table(schema.create_table_from_entity(comments::Entity))
      .await
  }

  async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    manager
      .drop_table(Table::drop().table(comments::Entity).to_owned())
      .await
  }
}
