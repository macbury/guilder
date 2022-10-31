use std::{collections::{HashMap}, sync::{Arc}, time::Duration};
use serde::{Deserialize, Serialize};
use anyhow::{Result};
use tokio::time::sleep;


use tokio::sync::RwLock;
use tokio::sync::mpsc::{channel, Sender, Receiver};
use crate::{Connection, Ticker, TradingViewEvent, Command};

const MAX_PER_GROUP : usize = 15;
type Sink = Sender<TradingViewEvent>;

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum SymbolType {
  Stock,
  Index,
  Cfd,
  Crypto,
  Futures,
  Economic,
  Forex,
  Bond
}

#[derive(Debug, Deserialize, Clone)]
pub struct SearchResult {
  pub symbol: String,
  pub description: String,
  pub prefix: Option<String>,
  pub exchange: String,
  pub currency_code: Option<String>,
  #[serde(alias="currency-logoid", alias = "logoid")]
  pub logo_id: Option<String>,
  #[serde(alias="base-currency-logoid")]
  pub base_logo_id: Option<String>,
  pub country: Option<String>,
  #[serde(alias="type")]
  pub kind: SymbolType
}

impl Into<Ticker> for SearchResult {
  fn into(self) -> Ticker {
    let symbol = voca_rs::strip::strip_tags(&self.symbol);
    if self.kind == SymbolType::Forex {
      Ticker::new(None, &symbol)
    } else {
      let prefix = self.prefix.or(Some(self.exchange)).unwrap();
      Ticker::new(Some(&prefix), &symbol)
    }
  }
}

struct AssetsGroup {
  observing: Vec<Ticker>,
  connection: Connection,
  sink: Sink,
}

impl AssetsGroup {
  pub async fn new(sink: &Sink) -> Result<Self> {
    let connection = Connection::new(sink).await?;
    Ok(
      Self {
        sink: sink.clone(),
        observing: Vec::new(),
        connection,
      }
    )
  }
  pub async fn start(&mut self) -> Result<()> {
    self.connection.start()?;
    Ok(())
  }

  pub async fn cleanup_dead(&mut self) -> Result<()> {
    if self.connection.is_dead().await {
      tracing::trace!("Cleaning up dead asset group");

      let mut connection = Connection::new(&self.sink).await?;
      connection.start()?;
      self.connection = connection;

      let tickers = self.observing.clone();
      self.observing.clear();

      for ticker in tickers {
        let ticker = ticker.clone();
        self.observe(ticker).await?;
      }
    } else {
      tracing::trace!("Asset group is not dead, skipping...");
    }
    Ok(())
  }

  pub fn contains(&self, ticker: &Ticker) -> bool {
    self.observing.contains(ticker)
  }

  #[tracing::instrument(level="debug", name = "forget", skip(self))]
  pub async fn forget(&mut self, ticker: &Ticker) -> Result<bool> {
    self.observing.retain(|t| t != ticker);

    let qid = self.connection.find_qid(ticker).await;

    if let Some(qid) = qid {
      tracing::debug!("Found qid {}, sending remove symbol command", qid);
      self.connection.send(Command::quote_remove_symbols(&qid, ticker)).await?;

      return Ok(true)
    }

    tracing::debug!("Missing qid...");
    Ok(false)
  }

  #[tracing::instrument(level="debug", name = "observe", skip(self))]
  pub async fn observe(&mut self, ticker: Ticker) -> Result<bool> {
    let chart_session = self.connection.generate_chart_id(&ticker).await;
    let quote_session = self.connection.generate_quote_session_id(&ticker).await;

    tracing::trace!("Chart session: {:?}", chart_session);
    tracing::trace!("Quote session: {:?}", quote_session);
    tracing::trace!("Ticker: {:?}", ticker);

    self.connection.send(Command::quote_create_session(&quote_session)).await?;
    self.connection.send(Command::chart_create_session(&chart_session)).await?;
    self.connection.send(Command::set_time_zone(&chart_session)).await?;

    let mut fields = vec![
      "base-currency-logoid".to_owned(),
      "ch".to_owned(),
      "chp".to_owned(),
      "bid".to_owned(),
      "ask".to_owned(),
      "country_code".to_owned(),
      "country".to_owned(),
      "currency-logoid".to_owned(),
      "currency_code".to_owned(),
      "description".to_owned(),
      "exchange".to_owned(),
      "is_tradable".to_owned(),
      "language".to_owned(),
      "local_description".to_owned(),
      "logoid".to_owned(),
      "original_name".to_owned(),
      "pro_name".to_owned(),
      "short_name".to_owned(),
      "type".to_owned(),
      "update_mode".to_owned(),
      "volume".to_owned(),
      "web_site_url".to_owned(),
      "business_description".to_owned(),
      "industry".to_owned(),
      "sector-i18n-en".to_owned()
    ];

    self.connection.send(
      Command::quote_add_symbols(
        &quote_session,
        ticker.clone()
      )?
    ).await?;

    self.connection.send(
      Command::resolve_symbol(
        &chart_session,
        ticker.clone()
      )?
    ).await?;

    self.connection.send(
      Command::quote_set_fields(
        &quote_session,
        &mut fields
      ),
    ).await?;

    self.connection.send(
      Command::quote_fast_symbols(
        &quote_session,
        ticker.clone()
      )?
    ).await?;

    self.connection.send(Command::create_series(&chart_session)).await?;
    self.observing.push(ticker.clone());

    Ok(true)
  }

  pub fn filled(&self) -> bool {
    self.observing.len() >= MAX_PER_GROUP
  }
}

#[derive(Clone)]
pub struct AssetsManager {
  groups: Arc<RwLock<Vec<AssetsGroup>>>,
  sink: Sink,
}

fn garbage_collection(instance: AssetsManager) {
  tokio::spawn(async move {
    let mut instance = instance;
    loop {
      let res = instance.cleanup().await;
      if let Err(err) = res {
        tracing::error!("Could not clean the connections: {:?}", err);
        sleep(Duration::from_secs(10)).await;
      } else {
        sleep(Duration::from_secs(1)).await;
      }
    }
  });
}

impl AssetsManager {
  pub fn new() -> (Self, Receiver<TradingViewEvent>) {
    let (sink, stream) = channel::<TradingViewEvent>(32);

    let instance = Self {
      groups: Arc::new(RwLock::new(Vec::new())),
      sink
    };

    garbage_collection(instance.clone());

    (instance, stream)
  }

  #[tracing::instrument]
  pub async fn search(name: &str) -> Result<Vec<SearchResult>> {
    tracing::debug!("Searching for: {}", name);
    let mut params = HashMap::new();
    params.insert("text", name);
    params.insert("hl", "1");
    params.insert("exchange", "");
    params.insert("lang", "en");
    params.insert("type", "");
    params.insert("domain", "production");

    let response = reqwest::Client::new()
      .get("https://symbol-search.tradingview.com/symbol_search/")
      .header("origin", "https://www.tradingview.com")
      .header("authority", "symbol-search.tradingview.com")
      .header("referer", "https://www.tradingview.com/")
      .header("user-agent", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/99.0.4844.83 Safari/537.36")
      .query(&params)
      .send()
      .await?
      //.text()
      .json::<Vec<SearchResult>>()
      .await?;
    tracing::debug!("Found: {} results", response.len());
    Ok(response)
  }

  pub async fn cleanup(&mut self) -> Result<()> {
    tracing::trace!("Checking groups for cleanup...");
    let mut groups = self.groups.write().await;

    for group in groups.iter_mut() {
      group.cleanup_dead().await?;
    }
    Ok(())
  }

  pub async fn forget(&mut self, ticker : Ticker) -> Result<bool> {
    let mut groups = self.groups.write().await;

    let group = groups.iter_mut().find(|c| c.contains(&ticker));

    if group.is_none() {
      tracing::trace!("Already stopped observing: {:?}", ticker);
      return Ok(true) // already observing
    }

    let group = group.unwrap();
    group.forget(&ticker).await?;

    Ok(true)
  }

  pub async fn observe(&mut self, ticker : Ticker) -> Result<bool> {
    let mut groups = self.groups.write().await;

    let group = groups.iter().find(|c| c.contains(&ticker));

    if let Some(_) = group {
      tracing::trace!("Already observing: {:?}", ticker);
      return Ok(false) // already observing
    }

    let free_group = groups.iter_mut().find(|c| !c.filled());

    if let Some(existing_group) = free_group {
      tracing::trace!("Observing with existing group: {:?}", ticker);

      return existing_group.observe(ticker.clone()).await
    } else {
      tracing::trace!("Creating new group for: {:?}", ticker);
      let mut group = AssetsGroup::new(&self.sink).await?;
      group.start().await?;
      group.observe(ticker.clone()).await?;
      groups.push(group);
      tracing::trace!("New group ready");
      return Ok(true)
    }
  }
}
