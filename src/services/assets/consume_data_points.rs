use crate::utils;
use anyhow::{Result, Ok};
use entity::import::ImportDataPoints;
use entity::{sea_orm::DatabaseConnection};
use entity::points::Kind;
use tokio::time::Instant;
use tracing::instrument;
use trading_view::commands::HistoryItem;
use std::collections::HashMap;
use std::time::Duration;
use trading_view::{TradingViewEvent, Ticker};

static FLUSH_AFTER_SECS : u64 = 10;

#[derive(Default, Debug, Copy, Clone)]
struct DataPoint {
  price: Option<f64>,
  volume: Option<f64>
}

pub struct TradingViewDataPointsConsumer {
  db: DatabaseConnection,
  points: HashMap<Ticker, DataPoint>,
  last_flush: Instant,
}

impl TradingViewDataPointsConsumer {
  pub fn new(db: DatabaseConnection) -> Self {
    Self { db, points: HashMap::new(), last_flush: Instant::now() } // i need access to database here.
  }

  /**
   * Return list of flushed tickers,
   */
  #[instrument(level="debug", skip_all)]
  pub async fn flush<'a>(&'a mut self) -> Result<Option<Vec<Ticker>>> {
    if self.last_flush.elapsed() < Duration::from_secs(FLUSH_AFTER_SECS) {
      return Ok(None)
    }

    self.last_flush = Instant::now();
    if self.points.is_empty() {
      tracing::trace!("Nothing to flush");
      return Ok(None)
    } else {
      tracing::trace!("Got data points: {:?}", self.points);
    }

    let mut tickers : Vec<Ticker> = Vec::new();
    let mut data_points = ImportDataPoints::new(&self.db, "Asset");
    let today = utils::today();

    let points = self.points.clone();
    self.points.clear();

    for (ticker, point) in &points {
      let resource_id = ticker.to_s();
      if let Some(price) = point.price {
        if price > 0.0 {
          data_points.add(resource_id.clone(), Kind::Price, today, price).await?;
        }
      }

      if let Some(volume) = point.volume {
        if volume > 0.0 {
          data_points.add(resource_id, Kind::Volume, today, volume).await?;
        }
      }

      tickers.push(ticker.clone());
    }

    data_points.commit().await?;

    Ok(Some(tickers))
  }

  #[instrument(level="debug", skip(self, points))]
  async fn import_points(&self, ticker: &Ticker, points: &Vec<HistoryItem>) -> Result<()> {
    tracing::debug!("Importing history: {} points", points.len());
    let mut data_points = ImportDataPoints::new(&self.db, "Asset");

    for point in points {
      let at = point.time.naive_utc().date();
      let resource_id = ticker.to_s();

      if point.volume > 0.0 {
        data_points.add(resource_id.clone(), Kind::Volume, at, point.volume).await?;
      }
      if point.close_price > 0.0 {
        data_points.add(resource_id, Kind::Price, at, point.close_price).await?;
      }
    }

    data_points.commit().await?;
    tracing::debug!("Done");
    Ok(())
  }

  pub async fn consume(&mut self, event: TradingViewEvent) -> Result<Option<Ticker>> {
    match event.clone() {
      TradingViewEvent::NewHistory(ticker, data) => {
        self.import_points(&ticker, &data).await?;
        return Ok(Some(ticker))
      }, // use different way of consuming events
      TradingViewEvent::Volume(ticker, volume) => {
        let point = self.points.entry(ticker.clone()).or_default();
        point.volume = Some(volume);

        return Ok(Some(ticker))
      },
      TradingViewEvent::Price(ticker, amount) => {
        let point = self.points.entry(ticker.clone()).or_default();
        point.price = Some(amount);
        return Ok(Some(ticker))
      },
      _ => {}
    }

    Ok(None)
  }
}

#[cfg(test)]
mod test {
  use anyhow::Context;
  use trading_view::{TradingViewEvent, Ticker};
  use crate::test::db;
  use super::*;

  async fn consumer() -> Result<(DatabaseConnection, TradingViewDataPointsConsumer)> {
    let db = db().await?;
    return Ok((db.clone(), TradingViewDataPointsConsumer::new(db.clone())));
  }

  #[tokio::test]
  async fn consume_bid() -> Result<()> {
    let (_db, mut consumer) = consumer().await?;
    consumer.consume(
      TradingViewEvent::Price(
        Ticker::new(Some("NASDAQ"), "TSLA"),
        12.2
      )
    )
    .await
    .with_context(|| "Could not consume event")?
    .unwrap();

    Ok(())
  }
}
