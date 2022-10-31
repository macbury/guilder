use std::collections::HashMap;

use anyhow::{Result, anyhow};
use serde::ser::{Serialize, SerializeStruct, SerializeMap};
use serde::Deserialize;
use serde_json::Value;
use tokio::sync::mpsc::Sender;
use chrono::{Utc, TimeZone, DateTime};

use crate::Ticker;

#[derive(Debug)]
pub enum Param {
  String(String),
  Number(i64),
  Hash(HashMap<String, String>)
}

impl From<&str> for Param {
  fn from(text: &str) -> Self {
    Param::String(text.to_owned())
  }
}

impl From<String> for Param {
  fn from(text: String) -> Self {
    Param::String(text.to_owned())
  }
}

impl TryFrom<Ticker> for Param {
  type Error = anyhow::Error;

  fn try_from(value: Ticker) -> Result<Self, Self::Error> {
    Ok(
      Param::String(value.to_s())
    )
  }
}

impl From<usize> for Param {
  fn from(num: usize) -> Self {
    Param::Number(num as i64)
  }
}

impl Serialize for Param {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where S: serde::Serializer {
    match self {
      Param::Number(val) => serializer.serialize_i64(*val),
      Param::String(val) => serializer.serialize_str(val),
      Param::Hash(_) => {
        let map = serializer.serialize_map(None)?;
        map.end()
      },
    }
  }
}

#[derive(Debug)]
pub struct Command {
  name : String,
  params: Option<Vec<Param>>
}

impl Command {

  pub fn set_auth_token() -> Self {
    Self {
      name: "set_auth_token".to_owned(),
      params: Some(vec!["unauthorized_user_token".into()])
    }
  }

  pub fn set_time_zone(chart_session_id: &str) -> Self {
    Self {
      name: "switch_timezone".to_owned(),
      params: Some(vec![chart_session_id.into(), "Etc/UTC".into()])
    }
  }

  pub fn set_data_quality_high() -> Self {
    Self {
      name: "set_data_quality".to_owned(),
      params: Some(vec!["high".into()])
    }
  }

  pub fn resolve_symbol(cid : &str, symbol : Ticker) -> Result<Self> {
    Ok(
      Self {
        name: "resolve_symbol".to_owned(),
        params: Some(vec![cid.into(), "sds_sym_1".into(), Self::symbol_sets(symbol)?])
      }
    )
  }
  // m: "create_series"
  // p: ["cs_x5v1pP1YTzVW", "sds_1", "s1", "sds_sym_1", "60", 300, ""]
  pub fn create_series(cid : &str) -> Self {
    Self {
      name: "create_series".to_owned(),
      params: Some(
        vec![
          cid.into(),
          "sds_1".into(),
          "s1".into(),
          "sds_sym_1".into(),
          "D".into(),
          300.into(),
          "ALL".into()
        ]
      )
    }
  }

  // m: "create_study"
  // p: ["cs_x5v1pP1YTzVW", "st1", "sessions_1", "sds_1", "Sessions@tv-basicstudies-156", {}]

  pub fn create_study(chart_session_id: &str, params: Vec<&str>) -> Self {
    let mut p : Vec<Param> = Vec::new();
    p.push(chart_session_id.into());
    for param in params {
      p.push(param.into());
    }
    p.push(Param::Hash(HashMap::new()));

    Self {
      name: "create_study".to_owned(),
      params: Some(p)
    }
  }

  pub fn chart_create_session(cid : &str) -> Self {
    Self {
      name: "chart_create_session".to_owned(),
      params: Some(vec![cid.into(), "".into()])
    }
  }

  pub fn quote_fast_symbols(qid : &str, ticker: Ticker) -> Result<Self> {
    Ok(
      Self {
        name: "quote_fast_symbols".to_owned(),
        params: Some(vec![qid.into(), Self::symbol_sets(ticker.clone())?, ticker.try_into()?])
      }
    )
  }

  pub fn quote_create_session(qid : &str) -> Self {
    Self {
      name: "quote_create_session".to_owned(),
      params: Some(vec![qid.into(), "".into()])
    }
  }

  pub fn quote_remove_symbols(qid : &str, ticker: &Ticker) -> Self {
    let ticker : String = ticker.into();
    Self {
      name: "quote_remove_symbols".to_owned(),
      params: Some(vec![qid.into(), ticker.into()])
    }
  }

  pub fn quote_set_fields(qid : &str, fields : &mut Vec<String>) -> Self {
    let mut params = vec![
      qid.into()
    ];

    for field in fields {
      let field = field;
      params.push(Param::String(field.clone()));
    }

    Self {
      name: "quote_set_fields".to_owned(),
      params: Some(params)
    }
  }

  fn symbol_sets(ticker : Ticker) -> Result<Param> {
    let symbol : String = ticker.try_into()?;
    Ok(format!("={{\"symbol\":\"{}\",\"adjustment\":\"splits\",\"session\":\"extended\"}}", symbol).into())
  }

  pub fn quote_add_symbols(qid : &str, symbol : Ticker) -> Result<Self> {
    Ok(
      Self {
        name: "quote_add_symbols".to_owned(),
        params: Some(
          vec![
            qid.into(),
            symbol.try_into()?
          ]
        )
      }
    )
  }
}

impl Serialize for Command {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where S: serde::Serializer
  {
    let mut state = serializer.serialize_struct("Color", 3)?;
    state.serialize_field("m", &self.name)?;

    if let Some(params) = &self.params {
      state.serialize_field("p", params)?;
    } else {
      state.serialize_field("p", &[0; 0])?;
    }

    state.end()
  }
}

impl TryInto<String> for Command {
  type Error = anyhow::Error;

  fn try_into(self) -> Result<String, Self::Error> {
    let serialized_cmd = serde_json::to_string(&self)?;

    Ok(format!("~m~{}~m~{}", serialized_cmd.len(), serialized_cmd))
  }
}

pub type ChartSessionId = String;

#[derive(Debug, PartialEq, serde::Serialize, Clone)]
pub enum TradingViewEvent {
  Price(Ticker, f64),
  Volume(Ticker, f64),
  Change(Ticker, f64),
  Exchange(Ticker, String),
  CurrencyCode(Ticker, String),
  Country(Ticker, String),
  Type(Ticker, String),
  NewHistory(Ticker, Vec<HistoryItem>),
  Logo(Ticker, String),
  Title(Ticker, String),
  WebsiteUrl(Ticker, String),
  Description(Ticker, String),
  CurrencyLogo(Ticker, String),
  BaseCurrencyLogo(Ticker, String),
  Isin(Ticker, String),
  AssetMetadataUpdated(Ticker, AssetMetadata),
  QuoteAccepted
}

#[derive(Deserialize, Debug, serde::Serialize)]
pub struct Response {
  #[serde(rename(deserialize = "m"))]
  pub name: String,
  #[serde(rename(deserialize = "p"))]
  pub params: serde_json::Value
}

#[derive(Debug, PartialEq, serde::Serialize, Clone)]
pub struct AssetMetadata {
  name: String,
  ticker: Ticker,
}

//use chrono::serde::ts_seconds_option;
#[derive(Debug, PartialEq, serde::Serialize, Clone, Copy)]
pub struct HistoryItem {
  pub time: DateTime<Utc>,
  pub close_price: f64,
  pub highest_price: f64,
  pub lowest_price: f64,
  pub opening_price: f64,
  pub volume: f64
}

fn extract_str(key : &str, options: &Value) -> Option<String> {
  return options.get(key)
    .and_then(|text| text.as_str())
    .and_then(|text| Some(text.to_owned()))
}

fn extract_float(key : &str, options: &Value) -> Option<f64> {
  return options.get(key).and_then(|value| value.as_f64())
}

impl Response {
  pub async fn to_timescale_update(&self, sink: &Sender<TradingViewEvent>, chart_sessions: &HashMap<String, Ticker>) -> Result<()> {
    let chart_session_id = self.params.get(0).and_then(|v| v.as_str());
    tracing::trace!("got timescale: {:?}", chart_session_id);

    if let Some(chart_session_id) = chart_session_id {
      let empty : &Vec<Value> = &Vec::new(); // TODO: better way?
      let history_items = self.params.get(1)
        .and_then(|v| v.get("sds_1"))
        .and_then(|v| v.get("s"))
        .and_then(|s| s.as_array())
        .map(|series| {
          let items : Vec<HistoryItem> = series
            .iter()
            .map(|row| {
              if let Some(rows) = row.get("v").and_then(|v| v.as_array()) {
                rows
              } else {
                empty
              }
            })
            .map(|v| {
              let secs = v.get(0).and_then(|v| v.as_f64()).unwrap_or_default();
              let opening_price = v.get(1).and_then(|v| v.as_f64()).unwrap_or_default();
              let highest_price = v.get(2).and_then(|v| v.as_f64()).unwrap_or_default();
              let lowest_price = v.get(3).and_then(|v| v.as_f64()).unwrap_or_default();
              let close_price = v.get(4).and_then(|v| v.as_f64()).unwrap_or_default();
              let volume = v.get(5).and_then(|v| v.as_f64()).unwrap_or_default();
              let time = Utc.timestamp(secs as i64, 0);
              //         1491202800, // date 2017-04-03 09:00:00 +0200
              //         4.63, // opening
              //         5.34, // highest price
              //         4.46, // lowest price
              //         5.17,// closing price?
              //         127042 // volume
              return HistoryItem {
                time,
                opening_price,
                highest_price,
                lowest_price,
                close_price,
                volume
              }
            })
            .collect();
            items
        });

        if let Some(history_items) = history_items {
          tracing::trace!("History items count: {}", history_items.len());
          let ticker = chart_sessions.get(chart_session_id);

          if let Some(ticker) = ticker {
            sink.send(
              TradingViewEvent::NewHistory(ticker.clone(), history_items)
            ).await?;
          } else {
            tracing::error!("Could not find ticker for chart_session_id: {}", chart_session_id);
          }
        }
    }

    Ok(())
  }

  pub async fn to_ticker_data(&self, sink: &Sender<TradingViewEvent>) -> Result<()> {
    if let Some(options) = self.params.get(1) {
      let ticker : Ticker = extract_str("n", options).unwrap_or_default().try_into()?;

      if let Some(value) = options.get("v") {
        let bid = value["bid"].as_f64();
        let ask = value["ask"].as_f64();

        if let (Some(bid), Some(ask)) = (bid, ask) {
          let price = (bid + ask) / 2.0;

          sink.send(
            TradingViewEvent::Price(ticker.clone(), price)
          ).await?;
        }

        if let Some(volume) = extract_float("volume", value) {
          sink.send(
            TradingViewEvent::Volume(ticker.clone(), volume)
          ).await?;
        }

        if let Some(change) = extract_float("change", value) {
          sink.send(
            TradingViewEvent::Change(ticker.clone(), change)
          ).await?;
        }

        if let Some(kind) = extract_str("type", value) {
          sink.send(
            TradingViewEvent::Type(ticker.clone(), kind)
          ).await?;
        }

        if let Some(currency_code) = extract_str("currency_code", value) {
          sink.send(
            TradingViewEvent::CurrencyCode(ticker.clone(), currency_code)
          ).await?;
        }

        if let Some(logoid) = extract_str("logoid", value) {
          sink.send(
            TradingViewEvent::Logo(ticker.clone(), format!("https://s3-symbol-logo.tradingview.com/{}--big.svg", logoid))
          ).await?;
        }

        if let Some(description) = extract_str("description", value) {
          sink.send(
            TradingViewEvent::Title(ticker.clone(), description)
          ).await?;
        }

        if let Some(web_site_url) = extract_str("web_site_url", value) {
          sink.send(
            TradingViewEvent::WebsiteUrl(ticker.clone(), web_site_url)
          ).await?;
        }

        if let Some(country) = extract_str("country_code", value) {
          sink.send(
            TradingViewEvent::Country(ticker.clone(), country)
          ).await?;
        }

        if let Some(exchange) = extract_str("exchange", value) {
          sink.send(
            TradingViewEvent::Exchange(ticker.clone(), exchange)
          ).await?;
        }

        if let Some(business_description) = extract_str("business_description", value) {
          sink.send(
            TradingViewEvent::Description(ticker.clone(), business_description)
          ).await?;
        }

        if let Some(currency_logo_id) = extract_str("currency-logoid", value) {
          sink.send(
            TradingViewEvent::CurrencyLogo(
              ticker.clone(),
              format!("https://s3-symbol-logo.tradingview.com/{}--big.svg", currency_logo_id)
            )
          ).await?;
        }

        if let Some(base_currency_logo_id) = extract_str("base-currency-logoid", value) {
          sink.send(
            TradingViewEvent::BaseCurrencyLogo(
              ticker.clone(),
              format!("https://s3-symbol-logo.tradingview.com/{}--big.svg", base_currency_logo_id)
            )
          ).await?;
        }
      }

      Ok(())
    } else {
      Err(anyhow!("Missing argument for params"))
    }
  }

  pub async fn to_symbol_resolved(&self, sink: &Sender<TradingViewEvent>) -> Result<()> {
    if let Some(options) = self.params.get(2) {
      let ticker = options.get("pro_name").and_then(|t| t.as_str());

      if let Some(ticker) = ticker {
        let ticker : Ticker = ticker.try_into()?;

        if let Some(isin) = options.get("isin").and_then(|t| t.as_str()) {
          sink.send(
            TradingViewEvent::Isin(ticker, isin.to_string())
          ).await?;
        }
      }
    }

    Ok(())
  }
}
