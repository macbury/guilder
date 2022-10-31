use entity::comments;
use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
  fn name(&self) -> &str {
    "m20220519_100706_create_comments_index"
  }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
  async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    let idx = sea_query::Index::create()
      .name("comments_association")
      .table(comments::Entity)
      .col(comments::Column::ModelId)
      .col(comments::Column::ModelType)
      .to_owned();

    manager.create_index(idx).await
  }

  async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    let idx = sea_query::Index::drop()
      .name("comments_association")
      .table(comments::Entity)
      .to_owned();

    manager.drop_index(idx).await
  }
}
