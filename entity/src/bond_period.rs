use sea_orm::entity::prelude::*;
use serde::Serialize;
use anyhow::{Result, anyhow};
use super::{BondModel, BondPerformanceModel};

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BondPeriod {
  pub index: u64,
  pub rate: f64,
  pub start_date: Date,
  pub end_date: Date
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BondPeriods(Vec<BondPeriod>);

impl TryFrom<(BondModel, Option<BondPerformanceModel>)> for BondPeriods {
  type Error = anyhow::Error;

  fn try_from((bond, performance): (BondModel, Option<BondPerformanceModel>)) -> Result<Self, Self::Error> {
    let performance = performance.ok_or(anyhow!("Missing bond performance model"))?;

    let mut result : Vec<BondPeriod> = vec![];
    let rates = performance.rates;
    let periods_start = bond.period_start.as_array().unwrap();
    let periods_end = bond.period_end.as_array().unwrap();

    for (index, period_start) in periods_start.iter().enumerate() {
      let start_date = period_start.as_str().expect("Could not cast period to string");
      let end_date = periods_end[index].as_str().expect("Missing matching period");
      let start_date = Date::parse_from_str(start_date, "%Y-%m-%d")?;
      let end_date = Date::parse_from_str(end_date, "%Y-%m-%d")?;
      let rate = rates[index].as_f64().expect("Missing rate");

      result.push(BondPeriod { index: index.try_into()?, start_date, end_date, rate })
    }

    Ok(BondPeriods(result))
  }
}
