use entity::points;
use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
  fn name(&self) -> &str {
    "m20220101_000007_create_points_upsert_index"
  }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
  async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    let idx = sea_query::Index::create()
      .name("points-upsert")
      .table(points::Entity)
      .col(points::Column::ResourceId)
      .col(points::Column::ResourceType)
      .col(points::Column::Date)
      .col(points::Column::Kind)
      .unique()
      .to_owned();

    manager.create_index(idx).await
  }

  async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    let idx = sea_query::Index::drop()
      .name("points-upsert")
      .table(points::Entity)
      .to_owned();

    manager.drop_index(idx).await
  }
}
