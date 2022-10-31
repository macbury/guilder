use entity::{BondModel, BondPerformanceModel};
use ::serde::Serialize;

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BondResource {
  #[serde(flatten)]
  pub bond: BondModel,
  pub performance: Option<BondPerformanceModel>
}

impl From<(BondModel, Option<BondPerformanceModel>)> for BondResource {
  fn from((bond, performance): (BondModel, Option<BondPerformanceModel>)) -> Self {
    Self { bond, performance }
  }
}

impl From<&(BondModel, Option<BondPerformanceModel>)> for BondResource {
  fn from((bond, performance): &(BondModel, Option<BondPerformanceModel>)) -> Self {
    Self { bond: bond.clone(), performance: performance.clone() }
  }
}
