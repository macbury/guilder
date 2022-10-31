use entity::{
  Order,
  SelectTwo,
  QuerySelect,
  JoinType,
  RelationTrait,
  QueryOrder, Asset, AssetPerformance, assets, categories, asset_performances,
};
use serde::{Deserialize};

#[derive(Debug, FromFormField, PartialEq, Deserialize)]
#[serde(rename_all="camelCase")]
pub enum SortBy {
  Name,
  Ticker,
  Price,
  DailyChange,
  CategoryId,
  YearlyPercentChange,
  YtdPercentChange,
  LowHighScore
}

impl SortBy {
  #[tracing::instrument(level="info", skip(query))]
  pub fn apply(&self, query : SelectTwo<Asset, AssetPerformance>, direction: Option<SortDirection>) -> SelectTwo<Asset, AssetPerformance> {
    let direction = direction.unwrap_or_default().into();
    tracing::info!("Apply sort to query");
    match self {
      Self::CategoryId => {
        query
          .join(JoinType::LeftJoin, assets::Relation::Category.def())
          .order_by(categories::Column::Name, direction)
      },
      Self::Price => query.order_by(asset_performances::Column::Price, direction),
      Self::Ticker => query.order_by(assets::Column::Id, direction),
      Self::DailyChange => query.order_by(asset_performances::Column::PercentChange, direction),
      Self::YearlyPercentChange => query.order_by(asset_performances::Column::YearlyPercentChange, direction),
      Self::YtdPercentChange => query.order_by(asset_performances::Column::YtdPercentChange, direction),
      Self::LowHighScore => query.order_by(asset_performances::Column::LowHighScore, direction),
      Self::Name => query.order_by(assets::Column::Name, direction)
    }
  }
}

#[derive(Debug, FromFormField, PartialEq, Deserialize)]
#[serde(rename_all="camelCase")]
pub enum SortDirection {
  Asc,
  Desc
}

impl Into<Order> for SortDirection {
  fn into(self) -> Order {
    match self {
      SortDirection::Desc => Order::Desc,
      SortDirection::Asc => Order::Asc
    }
  }
}

impl Default for SortDirection {
  fn default() -> Self {
    SortDirection::Asc
  }
}

impl Default for SortBy {
  fn default() -> Self {
    SortBy::Name
  }
}
