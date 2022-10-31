use serde::Serialize;
use std::fmt::Debug;

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Ticker {
  pub exchange: String,
  pub symbol: String,
  pub currency: bool
}

impl Serialize for Ticker {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where S: serde::Serializer {
    let ticker : String = self.into();
    serializer.serialize_str(&ticker)
  }
}

impl Debug for Ticker {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "\"{}\"", self.to_s())
  }
}

impl Ticker {
  pub fn new(exchange: Option<&str>, symbol: &str) -> Self {
    if let Some(exchange) = exchange {
      return Self {
        exchange: exchange.to_owned(),
        symbol: symbol.to_owned(),
        currency: false
      }
    } else {
      return Self {
        exchange: "".to_owned(),
        symbol: symbol.to_owned(),
        currency: true
      }
    }
  }

  pub fn to_s(&self) -> String {
    if self.currency {
      return format!("{}", self.symbol.to_ascii_uppercase())
    }

    format!("{}:{}", self.exchange.to_ascii_uppercase(), self.symbol.to_ascii_uppercase())
  }
}

impl Into<String> for Ticker {
  fn into(self) -> String {
    self.to_s()
  }
}

impl Into<String> for &Ticker {
  fn into(self) -> String {
    self.to_s()
  }
}

impl From<String> for Ticker {
  fn from(value: String) -> Self {
    let val: &str = value.as_ref();
    val.into()
  }
}

impl From<&str> for Ticker {
  fn from(value: &str) -> Self {
    let mut parts = value.split(":");

    let exchange = parts.next().unwrap_or_default().to_owned();
    let symbol = parts.next();

    if let Some(symbol) = symbol {
      let symbol = symbol.to_owned();

      return Self {
        exchange, symbol, currency: false
      }
    }

    Self {
      exchange: "".to_owned(),
      currency: true,
      symbol: exchange.to_owned()
    }
  }
}
