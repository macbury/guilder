use entity::{
  Order,
  SelectTwo,
  QuerySelect,
  JoinType,
  RelationTrait,
  BondPerformance,
  QueryOrder,
  Bond,
  bonds::{self, BondStatus},
  bond_performances,
  categories,
  wallets,
  accounts, QueryFilter, ColumnTrait
};
use serde::{Deserialize};

#[derive(Debug, FromFormField, PartialEq, Deserialize)]
#[serde(rename_all="camelCase")]
pub enum Scope {
  All,
  Active,
  Archived
}

impl Default for Scope {
  fn default() -> Self {
    Self::Active
  }
}

impl Scope {
  #[tracing::instrument(level="info", skip(query))]
  pub fn apply(&self, query : SelectTwo<Bond, BondPerformance>) -> SelectTwo<Bond, BondPerformance> {
    tracing::info!("Apply scope");

    match self {
      Self::All => query,
      Self::Archived => query.filter(bonds::Column::Status.eq(BondStatus::Archived)),
      Self::Active => query.filter(bonds::Column::Status.eq(BondStatus::Active))
    }
  }
}

#[derive(Debug, FromFormField, PartialEq, Deserialize)]
#[serde(rename_all="camelCase")]
pub enum SortBy {
  Emission,
  StartDate,
  EndDate,
  Price,
  CategoryId,
  AccountId,
  WalletId,
  CurrentRate,
  InterestDaysLeft,
  Shares,
  StartPrice,
  DayPriceChange,
  DayPercentChange,
  PercentChange,
  BuyoutPrice,
  PriceDate
}

impl SortBy {
  #[tracing::instrument(level="info", skip(query))]
  pub fn apply(&self, query : SelectTwo<Bond, BondPerformance>, direction: Option<SortDirection>) -> SelectTwo<Bond, BondPerformance> {
    let direction = direction.unwrap_or_default().into();
    tracing::info!("Apply sort to query");
    match self {
      Self::BuyoutPrice => query.order_by(bond_performances::Column::BuyoutPrice, direction),
      Self::PriceDate => query.order_by(bond_performances::Column::PriceDate, direction),
      Self::PercentChange => query.order_by(bond_performances::Column::PercentChange, direction),
      Self::DayPercentChange => query.order_by(bond_performances::Column::DayPercentChange, direction),
      Self::DayPriceChange => query.order_by(bond_performances::Column::DayPriceChange, direction),
      Self::AccountId => {
        query
          .join(JoinType::LeftJoin, bonds::Relation::Account.def())
          .order_by(accounts::Column::Name, direction)
      },
      Self::Shares => query.order_by(bond_performances::Column::Shares, direction),
      Self::InterestDaysLeft => query.order_by(bond_performances::Column::InterestDaysLeft, direction),
      Self::CurrentRate => query.order_by(bond_performances::Column::CurrentRate, direction),
      Self::StartDate => query.order_by(bonds::Column::StartDate, direction),
      Self::EndDate => query.order_by(bonds::Column::EndDate, direction),
      Self::CategoryId => {
        query
          .join(JoinType::LeftJoin, bonds::Relation::Category.def())
          .order_by(categories::Column::Name, direction)
      },
      Self::WalletId => {
        query
          .join(JoinType::LeftJoin, bonds::Relation::Wallet.def())
          .order_by(wallets::Column::Name, direction)
      },
      Self::Price => query.order_by(bond_performances::Column::Price, direction),
      Self::StartPrice => query.order_by(bond_performances::Column::StartPrice, direction),
      Self::Emission => query.order_by(bonds::Column::Emission, direction)
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
    SortBy::Emission
  }
}
