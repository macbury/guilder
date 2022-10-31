use entity::{AssetModel, PointModel, AssetPerformanceModel};
use serde::Serialize;
use trading_view::Ticker;
use chrono::NaiveDate;
use entity::points::Kind;

#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AssetDetails {
  pub id: String,
  pub category_id: Option<i64>,
  pub name: Option<String>,
  pub description: Option<String>,
  pub website_url: Option<String>,
  pub isin: Option<String>,
  pub currency: Option<String>,
  pub logo_url: Option<String>,
  pub country: Option<String>,
  pub secondary_logo_url: Option<String>,
  pub exchange: Option<String>,
  pub country_logo: Option<String>,
  pub performance: Option<AssetPerformanceModel>
}

impl From<AssetModel> for AssetDetails {
  fn from(model: AssetModel) -> Self {
    Self {
      country_logo: model.country_logo_url(),
      secondary_logo_url: model.secondary_logo_url(),
      id: model.id,
      name: model.name,
      description: model.description,
      website_url: model.website_url,
      logo_url: model.logo_url,
      isin: model.isin,
      currency: model.currency,
      country: model.country,
      exchange: model.exchange,
      category_id: model.category_id,
      performance: None,
    }
  }
}

impl From<&AssetModel> for AssetDetails {
  fn from(model: &AssetModel) -> Self {
    let model = model.clone();
    model.into()
  }
}

impl From<(AssetModel, Option<AssetPerformanceModel>)> for AssetDetails {
  fn from(models: (AssetModel, Option<AssetPerformanceModel>)) -> Self {
    let (asset, asset_stats) = models;
    let mut asset_details : AssetDetails = asset.into();
    asset_details.performance = asset_stats.map(|s| {
      return s.into();
    });
    asset_details
  }
}

impl From<&(AssetModel, Option<AssetPerformanceModel>)> for AssetDetails {
  fn from(models: &(AssetModel, Option<AssetPerformanceModel>)) -> Self {
    let models = models.clone();
    models.into()
  }
}

#[derive(Serialize)]
pub struct DataPoint(Kind, Ticker, NaiveDate, f64);

impl DataPoint {
  pub fn build(point : &PointModel) -> Self {
    Self(point.kind, point.ticker(), point.date, point.value)
  }
}

impl From<PointModel> for DataPoint {
  fn from(point: PointModel) -> Self {
    Self::build(&point)
  }
}

impl From<&PointModel> for DataPoint {
  fn from(point: &PointModel) -> Self {
    Self::build(point)
  }
}
