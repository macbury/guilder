use entity::Comment;
use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
  fn name(&self) -> &str {
    "m20220619_104537_drop_comments_foregin"
  }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
  async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    manager.drop_foreign_key(
      ForeignKey::drop()
        .table(Comment)
        .name("fk-comments-model_id")
        .to_owned()
    ).await
  }

  async fn down(&self, _manager: &SchemaManager) -> Result<(), DbErr> {
    todo!()
  }
}
