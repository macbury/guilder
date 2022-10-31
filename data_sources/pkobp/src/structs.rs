use std::{str::FromStr, fmt::Display};
use chronoutil::{RelativeDuration, DateRule};
use regex::Regex;
use crypto::md5::Md5;
use crypto::digest::Digest;
use anyhow::{Result, anyhow};
use chrono::{NaiveDate, Datelike, Utc};
use rust_decimal::{prelude::{FromPrimitive, ToPrimitive}, Decimal};

use rusty_money::{Money, iso::{self, Currency}, Round};

#[derive(Debug)]
pub struct PKOBPBondRates(Vec<f64>);

impl Into<Vec<f64>> for PKOBPBondRates {
  fn into(self) -> Vec<f64> {
    self.into_inner()
  }
}

impl FromStr for PKOBPBondRates {
  type Err = anyhow::Error;

  fn from_str(text: &str) -> Result<Self, Self::Err> {
    let regex = Regex::new(r"(\d{1,2}\.\d{1,4}|(\d{1,2}))%")?;
    let mut rates = Vec::new();

    for caps in regex.captures_iter(text) {
      let rate_text = &caps[1];
      let rate = str::parse::<f64>(rate_text)?;
      rates.push(rate);
    }

    if rates.len() == 0 {
      return Err(anyhow::anyhow!("Missing rates"));
    }

    Ok(Self(rates))
  }
}

impl PKOBPBondRates {
  pub fn into_inner(self) -> Vec<f64> {
    return self.0
  }
}

pub struct PeriodInterest<'a> {
  pub index: u64,
  pub start_date : NaiveDate,
  pub end_date : NaiveDate,
  pub interest : Money<'a, Currency>,
  pub capital : Money<'a, Currency>,
  pub total_capital :  Money<'a, Currency>,
  pub rate : f64
}

impl<'a> PeriodInterest<'a> {
  pub fn active_or_past(&self) -> bool {
    self.start_date <= Utc::now().naive_local().date()
  }
}

pub type DayInterest<'a> = (NaiveDate, Money<'a, Currency>, f64, u64);

#[derive(Debug, Clone)]
pub struct PolishPeso<'a>(Money<'a, Currency>);

impl<'a> PolishPeso<'a> {
  pub fn into_inner(self) -> Money<'a, Currency> {
    return self.0
  }
}

impl<'a> FromStr for PolishPeso<'a> {
  type Err = anyhow::Error;

  fn from_str(text: &str) -> Result<Self, Self::Err> {
    let amount_regexp = Regex::new(r"(?P<amount>[\d ]+,\d{2})")?;
    let matches = amount_regexp.captures(text)
      .ok_or(anyhow::anyhow!("Could not parse amount"))?;
    let text_amount = &matches["amount"].replace(" ", "");
    let amount = Money::from_str(text_amount, iso::PLN)?;
    Ok(Self(amount))
  }
}

#[derive(Debug, Clone)]
pub struct PKOBPBond<'a> {
  pub account_id: &'a str,
  pub kind: BondKind,
  pub emission: String,
  pub shares: i32,
  pub start_price: Money<'a, Currency>,
  pub current_price: Money<'a, Currency>,
  pub buyout_date: NaiveDate,
  pub rates: Vec<f64>,
}

impl<'a> Default for PKOBPBond<'a> {
  fn default() -> Self {
    Self {
      account_id: Default::default(),
      kind: BondKind::EDO,
      emission: Default::default(),
      shares: Default::default(),
      start_price: Money::from_minor(0, iso::PLN),
      current_price: Money::from_minor(0, iso::PLN),
      buyout_date: NaiveDate::from_yo(2015, 1),
      rates: Default::default()
    }
  }
}

impl<'a> PKOBPBond<'a> {
  pub fn simulate_interest(&self) -> bool {
    self.kind == BondKind::OTS
  }

  pub fn id(&self) -> String {
    let mut hasher = Md5::new();
    hasher.input_str("guilder");
    hasher.input_str(self.account_id);
    hasher.input_str(&self.emission);

    let days = self.buyout_date.num_days_from_ce();
    hasher.input_str(&format!("days from ce {}", days));

    hasher.result_str()
  }

  pub fn start_date(&self) -> NaiveDate {
    let months = RelativeDuration::months(-self.kind.maturity_period());
    self.buyout_date + months
  }

  pub fn interest_periods(&self) -> Vec<(NaiveDate, NaiveDate)> {
    let mut periods = Vec::new();

    let months = RelativeDuration::months(self.kind.interest_period());
    let mut rules = DateRule::new(self.start_date(), months)
      .with_end(self.buyout_date + RelativeDuration::days(1));

    let mut prev_date = rules.next()
      .expect("Start date is invalid");

    while let Some(date) = rules.next() {
      periods.push((prev_date, date));
      prev_date = date;
    }
    periods
  }

  pub fn all_interests(&self) -> Vec<PeriodInterest<'a>> {
    let mut interests = Vec::new();
    let period_interest_rate_div = self.kind.period_interest_rate_div();

    let mut capital = self.start_price.clone();
    for (period, (start, end)) in self.interest_periods().iter().enumerate() {
      let rate = self.rates.get(period);
      if rate.is_none() {
        break;
      }
      let rate = rate.unwrap();

      let period_rate = Decimal::from_f64(rate / period_interest_rate_div / 100.0)
        .expect("Could not calculate period rate");

      let interest = capital.clone() * period_rate;
      let current_interest = interest.clone() + capital.clone();

      let period_interest = PeriodInterest {
        index: period.to_u64().expect("Could not convert period to index, this"),
        start_date: *start,
        end_date: *end,
        interest: interest.clone(),
        capital: current_interest,
        rate: *rate,
        total_capital: capital.clone()
      };

      if period_interest.active_or_past() {
        interests.push(period_interest);

        if self.kind.capitalize() {
          capital += interest;
        }
      }
    }
    return interests;
  }

  pub fn early_buyout_price(&self, period: u64, current_price: &'a Money<Currency>) -> Money<'a, Currency> {
    let commission = self.kind.early_buyout_commission() * self.shares;
    let price = current_price.clone() - commission;

    if (period == 0 && price < self.start_price) || self.kind == BondKind::OTS {
      return self.start_price.clone()
    } else {
      return price
    }
  }

  /**
   * Calculate interests earned to current day, returns tuple with: day, amount, and rate for that day
   */
  pub fn past_interests_to(&self, current_day: &NaiveDate) -> Vec<DayInterest<'a>> {
    let mut interests = Vec::new();
    let each_day = RelativeDuration::days(1);
    let periods = self.all_interests();

    for period_interest in periods {
      let mut period_range = DateRule::new(period_interest.start_date, each_day).with_end(period_interest.end_date);
      let total_period_days : f64 = (period_interest.end_date.num_days_from_ce() - period_interest.start_date.num_days_from_ce()).into();
      while let Some(period_date) = period_range.next() {
        if &period_date > current_day {
          return interests
        }

        let day_of_period : f64 = (period_date.num_days_from_ce() - period_interest.start_date.num_days_from_ce()).into();
        let progress = Decimal::from_f64(day_of_period / total_period_days).expect("Could not calculate progress for period");
        let current_interest = period_interest.interest.clone() * progress + period_interest.total_capital.clone();
        let current_interest = current_interest.round(2, Round::HalfEven);

        let interest = (period_date, current_interest, period_interest.rate, period_interest.index);
        interests.push(interest);
      }
    }
    return interests
  }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum BondKind {
  OTS,
  DOS,
  TOZ,
  COI,
  EDO,
  ROS,
  ROD,
  ROR,
  DOR
}

pub type NumberOfMonths = i32;

impl Into<String> for BondKind {
  fn into(self) -> String {
    match self {
      Self::OTS => "OTS".to_owned(),
      Self::DOS => "DOS".to_owned(),
      Self::TOZ => "TOZ".to_owned(),
      Self::COI => "COI".to_owned(),
      Self::EDO => "EDO".to_owned(),
      Self::ROS => "ROS".to_owned(),
      Self::ROD => "ROD".to_owned(),
      Self::ROR => "ROR".to_owned(),
      Self::DOR => "DOR".to_owned(),
    }
  }
}

impl BondKind {
  pub fn name(&self) -> &str {
    match self {
      Self::OTS => "Obligacje 3-miesięczne OTS",
      Self::ROR => "Obligacje roczne ROR",
      Self::DOR => "Obligacje 2-letnie DOR",
      Self::TOZ => "Obligacje 3-letnie TOZ",
      Self::COI => "Obligacje 4-letnie COI",
      Self::EDO => "Obligacje 10-letnie EDO",
      Self::DOS => "Obligacje 2-letnie DOS",
      Self::ROS => "Obligacje 6-letnie ROS",
      Self::ROD => "Obligacje 12-letnie ROD"
    }
  }

  pub fn early_buyout_commission<'a>(&self) -> Money<'a, Currency> {
    match self {
      Self::OTS => Money::from_minor(0, iso::PLN),
      Self::ROR => Money::from_minor(50, iso::PLN),
      Self::TOZ | Self::DOR | Self::COI | Self::ROS  => Money::from_minor(70, iso::PLN),
      Self::EDO | Self::ROD | Self::DOS => Money::from_minor(200, iso::PLN)
    }
  }

  pub fn capitalize(&self) -> bool {
    *self == Self::EDO
  }

  /**
   * Number of months you get paid interest
   */
  pub fn interest_period(&self) -> NumberOfMonths {
    match self {
      Self::OTS => 3,
      Self::ROR => 1,
      Self::DOR => 1,
      Self::TOZ => 6,
      Self::COI => 12,
      Self::EDO => 12,
      Self::DOS => 24,
      Self::ROS => 72,
      Self::ROD => 144
    }
  }

  pub fn period_interest_rate_div(&self) -> f64 {
    match self {
      Self::OTS => 4.0,
      Self::ROR => 12.0,
      Self::DOR => 12.0,
      Self::TOZ => 2.0,
      _ => 1.0
    }
  }

  /**
   * Number of months after bond is buyed out
   */
  pub fn maturity_period(&self) -> NumberOfMonths {
    match self {
      Self::OTS => 3,
      Self::ROR => 12,
      Self::DOR => 24,
      Self::TOZ => 36,
      Self::COI => 48,
      Self::EDO => 120,
      Self::DOS => 24,
      Self::ROS => 72,
      Self::ROD => 144
    }
  }
}

impl Display for BondKind {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.name())
  }
}

impl TryFrom<String> for BondKind {
  type Error = anyhow::Error;

  fn try_from(value: String) -> Result<Self, Self::Error> {
    let t : &str = value.as_ref();
    t.try_into()
  }
}

impl TryFrom<&str> for BondKind {
  type Error = anyhow::Error;

  fn try_from(value: &str) -> Result<Self, Self::Error> {
    match value {
      "ROR" => Ok(Self::ROR),
      "DOR" => Ok(Self::DOR),
      "OTS" => Ok(Self::OTS),
      "DOS" => Ok(Self::DOS),
      "TOZ" => Ok(Self::TOZ),
      "COI" => Ok(Self::COI),
      "EDO" => Ok(Self::EDO),
      "ROS" => Ok(Self::ROS),
      "ROD" => Ok(Self::ROD),
      value => Err(anyhow!("Could not match: {}", value))
    }
  }
}

#[cfg(test)]
mod tests {
  use chrono::NaiveDate;
  use rust_decimal_macros::dec;
  use rusty_money::{Money, iso};

  use super::{PolishPeso, PKOBPBondRates, BondKind, PKOBPBond};

  #[test]
  fn it_calculates_early_buyout_price_for_edo() {
    let mut bond = PKOBPBond {
      kind: BondKind::EDO,
      emission: "EDO0431".to_owned(),
      shares: 1,
      start_price: Money::from_major(100, iso::PLN),
      current_price: Money::from_major(103, iso::PLN),
      ..Default::default()
    };

    assert_eq!(bond.early_buyout_price(0, &bond.current_price), Money::from_major(101 , iso::PLN));
    assert_eq!(bond.early_buyout_price(1, &bond.current_price), Money::from_major(101 , iso::PLN));

    bond.current_price = Money::from_major(101, iso::PLN);
    assert_eq!(bond.early_buyout_price(0, &bond.current_price), Money::from_major(100 , iso::PLN));
    assert_eq!(bond.early_buyout_price(1, &bond.current_price), Money::from_major(99 , iso::PLN));
  }

  #[test]
  fn it_calculates_edo_interests() {
    let bond = PKOBPBond {
      kind: BondKind::EDO,
      emission: "EDO0431".to_owned(),
      buyout_date: NaiveDate::from_ymd(2031, 4,16),
      rates: vec![1.7f64, 9.5f64],
      start_price: Money::from_major(100, iso::PLN),
      ..Default::default()
    };

    let interests = bond.all_interests();
    let period_interest = interests.first().unwrap();

    assert_eq!(interests.len(), 2);

    assert_eq!(period_interest.rate, 1.7);
    assert_eq!(period_interest.capital, Money::from_minor(101_70 , iso::PLN));
    assert_eq!(period_interest.interest, Money::from_minor(1_70 , iso::PLN));
    assert_eq!(period_interest.start_date, NaiveDate::from_ymd(2021, 4, 16));
    assert_eq!(period_interest.end_date, NaiveDate::from_ymd(2022, 4, 16));

    let period_interest = interests.get(1).unwrap();

    assert_eq!(period_interest.rate, 9.5);
    assert_eq!(period_interest.capital, Money::from_str("111,361500", iso::PLN).unwrap());
    assert_eq!(period_interest.interest, Money::from_str("9,661500", iso::PLN).unwrap());
    assert_eq!(period_interest.start_date, NaiveDate::from_ymd(2022, 4, 16));
    assert_eq!(period_interest.end_date, NaiveDate::from_ymd(2023, 4, 16));
  }

  #[test]
  fn it_calculates_edo_rates() {
    let bond = PKOBPBond {
      kind: BondKind::EDO,
      emission: "EDO0431".to_owned(),
      buyout_date: NaiveDate::from_ymd(2031, 4,16),
      rates: vec![1.7f64, 9.5f64],
      start_price: Money::from_major(100, iso::PLN),
      ..Default::default()
    };

    let to = NaiveDate::from_ymd(2022, 6, 8);

    let interests = bond.past_interests_to(&to);
    let (date, amount, rate, _period) = interests.last().unwrap();

    assert_eq!(interests.len(), 419);
    assert_eq!(*rate, 9.5);
    assert_eq!(*amount, Money::from_minor(103_10 , iso::PLN));
    assert_eq!(*date, NaiveDate::from_ymd(2022, 6, 8));
  }

  #[test]
  fn it_calculates_toz_rates() {
    let bond = PKOBPBond {
      kind: BondKind::TOZ,
      emission: "TOZ0125".to_owned(),
      buyout_date: NaiveDate::from_ymd(2025, 1,13),
      rates: vec![1.1f64],
      start_price: Money::from_major(100, iso::PLN),
      ..Default::default()
    };

    let to = NaiveDate::from_ymd(2022, 6, 8);

    let interests = bond.past_interests_to(&to);
    let (date, amount, rate, _period) = interests.last().unwrap();

    assert_eq!(interests.len(), 147);
    assert_eq!(*amount, Money::from_minor(100_44, iso::PLN));
    assert_eq!(*rate, 1.1);
    assert_eq!(*date, NaiveDate::from_ymd(2022, 6, 8));
  }

  #[test]
  fn it_calculates_toz_interests() {
    let bond = PKOBPBond {
      kind: BondKind::TOZ,
      emission: "TOZ0125".to_owned(),
      buyout_date: NaiveDate::from_ymd(2025, 1,13),
      rates: vec![1.1f64],
      start_price: Money::from_major(100, iso::PLN),
      ..Default::default()
    };

    let interests = bond.all_interests();
    let period_interest = interests.last().unwrap();

    assert_eq!(interests.len(), 1);
    assert_eq!(period_interest.capital, Money::from_minor(100_55, iso::PLN));
    assert_eq!(period_interest.interest, Money::from_minor(55, iso::PLN));
    assert_eq!(period_interest.rate, 1.1);
    assert_eq!(period_interest.end_date, NaiveDate::from_ymd(2022, 7, 13));
    assert_eq!(period_interest.start_date, NaiveDate::from_ymd(2022, 1, 13));
  }

  #[test]
  fn it_calculates_coi_rates() {
    let bond = PKOBPBond {
      kind: BondKind::COI,
      emission: "COI1124".to_owned(),
      buyout_date: NaiveDate::from_ymd(2024, 11, 2),
      rates: vec![1.3f64, 6.65f64],
      start_price: Money::from_major(100, iso::PLN),
      ..Default::default()
    };

    let to = NaiveDate::from_ymd(2022, 6, 8);

    let interests = bond.past_interests_to(&to);
    let (date, amount, rate, _period) = interests.last().unwrap();

    assert_eq!(interests.len(), 584);
    assert_eq!(*rate, 6.65);
    assert_eq!(*date, NaiveDate::from_ymd(2022, 6, 8));
    assert_eq!(*amount, Money::from_minor(10397, iso::PLN));

    let (date, amount, rate, _period) = &interests[364];
    assert_eq!(*rate, 1.3);
    assert_eq!(*date, NaiveDate::from_ymd(2021, 11, 1));
    assert_eq!(*amount, Money::from_minor(10130, iso::PLN));

    let (date, amount, rate, _period) = &interests[365];
    assert_eq!(*rate, 6.65);
    assert_eq!(*date, NaiveDate::from_ymd(2021, 11, 2));
    assert_eq!(*amount, Money::from_minor(10000, iso::PLN));
  }

  #[test]
  fn it_calculates_coi_interests() {
    let bond = PKOBPBond {
      kind: BondKind::COI,
      emission: "COI1124".to_owned(),
      buyout_date: NaiveDate::from_ymd(2024, 11, 2),
      rates: vec![1.3f64, 6.65f64],
      start_price: Money::from_major(100, iso::PLN),
      ..Default::default()
    };

    let interests = bond.all_interests();
    let period_interest = interests.first().unwrap();

    assert_eq!(interests.len(), 2);
    assert_eq!(period_interest.rate, 1.3);
    assert_eq!(period_interest.start_date, NaiveDate::from_ymd(2020, 11, 2));
    assert_eq!(period_interest.end_date, NaiveDate::from_ymd(2021, 11, 2));
    assert_eq!(period_interest.capital, Money::from_minor(10130, iso::PLN));

    let period_interest = interests.get(1).unwrap();

    assert_eq!(period_interest.rate, 6.65);
    assert_eq!(period_interest.start_date, NaiveDate::from_ymd(2021, 11, 2));
    assert_eq!(period_interest.end_date, NaiveDate::from_ymd(2022, 11, 2));
    assert_eq!(period_interest.capital, Money::from_minor(10665, iso::PLN));
  }

  #[test]
  fn it_calculates_yearly_periods() {
    let bond = PKOBPBond {
      kind: BondKind::COI,
      buyout_date: NaiveDate::from_ymd(2023, 3, 15),
      ..Default::default()
    };

    assert_eq!(bond.interest_periods(), vec![
      (NaiveDate::from_ymd(2019, 3, 15), NaiveDate::from_ymd(2020, 3, 15)),
      (NaiveDate::from_ymd(2020, 3, 15), NaiveDate::from_ymd(2021, 3, 15)),
      (NaiveDate::from_ymd(2021, 3, 15), NaiveDate::from_ymd(2022, 3, 15)),
      (NaiveDate::from_ymd(2022, 3, 15), NaiveDate::from_ymd(2023, 3, 15))
    ]);
  }

  #[test]
  fn it_calculates_one_periods() {
    let bond = PKOBPBond {
      kind: BondKind::OTS,
      buyout_date: NaiveDate::from_ymd(2023, 3, 1),
      ..Default::default()
    };

    assert_eq!(bond.interest_periods(), vec![
      (NaiveDate::from_ymd(2022, 12, 1), NaiveDate::from_ymd(2023, 3, 1)),
    ]);
  }

  #[test]
  fn it_calculates_start_date() {
    let bond = PKOBPBond {
      kind: BondKind::COI,
      buyout_date: NaiveDate::from_ymd(2012, 1, 1),
      ..Default::default()
    };

    assert_eq!(bond.start_date(), NaiveDate::from_ymd(2008, 1, 1));

    let bond = PKOBPBond {
      kind: BondKind::ROR,
      buyout_date: NaiveDate::from_ymd(2012, 2, 12),
      ..Default::default()
    };

    assert_eq!(bond.start_date(), NaiveDate::from_ymd(2011, 2, 12));

    let bond = PKOBPBond {
      kind: BondKind::OTS,
      buyout_date: NaiveDate::from_ymd(2012, 2, 12),
      ..Default::default()
    };

    assert_eq!(bond.start_date(), NaiveDate::from_ymd(2011, 11, 12));
  }

  #[test]
  fn it_generates_uid() {
    let bond = PKOBPBond {
      account_id: "12345",
      emission: "EDO123".to_string(),
      buyout_date: NaiveDate::from_ymd(2012, 1, 1),
      ..Default::default()
    };

    assert_eq!(bond.id(), "4ebfafbeadd67e3b7f3736acc94a19f3");

    let bond = PKOBPBond {
      account_id: "account_id",
      emission: "EDO123".to_string(),
      buyout_date: NaiveDate::from_ymd(2012, 1, 1),
      ..Default::default()
    };

    assert_eq!(bond.id(), "ec9bedb65540d50c0e3517d0a185bf13");

    let bond = PKOBPBond {
      account_id: "account_id",
      emission: "EDO".to_string(),
      buyout_date: NaiveDate::from_ymd(2012, 1, 1),
      ..Default::default()
    };

    assert_eq!(bond.id(), "bd0a5a9685e44c69969b46b9aa3d2637");

    let bond = PKOBPBond {
      account_id: "account_id",
      emission: "EDO".to_string(),
      buyout_date: NaiveDate::from_ymd(2011, 1, 1),
      ..Default::default()
    };

    assert_eq!(bond.id(), "f1da47ed305ecf2f4adf3f0e2a3eaa42");
  }

  #[test]
  fn it_parses_enum() -> anyhow::Result<()> {
    let bond : BondKind = "ROR".try_into()?;
    assert_eq!(bond, BondKind::ROR);

    let bond : Result<BondKind, _> = "sss".try_into();
    assert!(bond.is_err());
    Ok(())
  }

  #[test]
  fn it_formats_enum() -> anyhow::Result<()> {
    assert_eq!("Obligacje 4-letnie COI", format!("{}", BondKind::COI));
    Ok(())
  }

  #[test]
  fn it_parses_valid_amounts() -> anyhow::Result<()> {
    let peso : PolishPeso = str::parse("1 200,22")?;
    assert_eq!(peso.into_inner().amount(), &dec!(1200.22));

    let peso : PolishPeso = str::parse("1,00")?;
    assert_eq!(peso.into_inner().amount(), &dec!(1.00));

    let peso : PolishPeso = str::parse("100,01")?;
    assert_eq!(peso.into_inner().amount(), &dec!(100.01));
    Ok(())
  }

  #[test]
  fn it_fails_amounts() -> anyhow::Result<()> {
    assert!(str::parse::<PolishPeso>("sss").is_err());
    assert!(str::parse::<PolishPeso>("").is_err());
    assert!(str::parse::<PolishPeso>("0").is_err());
    Ok(())
  }

  #[test]
  fn it_transforms_multiple_rates_into_one() -> anyhow::Result<()> {
    let rates = str::parse::<PKOBPBondRates>("okres 1 oprocentowanie 3.1%")?;

    assert_eq!(rates.into_inner(), vec![3.1]);
    Ok(())
  }

  #[test]
  fn it_transforms_single_rates_into_one() -> anyhow::Result<()> {
    let rates = str::parse::<PKOBPBondRates>("okres 1 oprocentowanie 1.3%\nokres 2 oprocentowanie 6.65%")?;

    assert_eq!(rates.into_inner(), vec![1.3, 6.65]);
    Ok(())
  }

  #[test]
  fn it_transforms_single_rates_into_single_digit() -> anyhow::Result<()> {
    let rates = str::parse::<PKOBPBondRates>("okres 1 oprocentowanie 1.3%\nokres 2 oprocentowanie 6%")?;

    assert_eq!(rates.into_inner(), vec![1.3, 6.0]);
    Ok(())
  }

  #[test]
  fn it_fails_rates() -> anyhow::Result<()> {
    assert!(str::parse::<PKOBPBondRates>("").is_err());
    assert!(str::parse::<PKOBPBondRates>("boom").is_err());
    assert!(str::parse::<PKOBPBondRates>("wwww").is_err());
    Ok(())
  }

}
