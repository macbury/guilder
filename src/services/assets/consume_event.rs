use anyhow::{Result, Ok};
use entity::{sea_orm::DatabaseConnection};
use entity::assets::{ActiveModel, Model};
use entity::{Asset, EntityTrait, Set, ActiveModelTrait};
use trading_view::{TradingViewEvent, Ticker};

/**
 * Transform TradingView event into Asset Entity
 */
pub struct TradingViewEventConsumer {
  db: DatabaseConnection
}

impl TradingViewEventConsumer {
  pub fn new(db: DatabaseConnection) -> Self {
    Self { db } // i need access to database here.
  }

  async fn find_asset(&self, ticker: Ticker) -> Result<ActiveModel> {
    let id : String = ticker.try_into()?;
    let asset = Asset::find_by_id(id.clone())
      .one(&self.db)
      .await?
      .and_then(|model| {
        let m : ActiveModel = model.into();
        return Some(m);
      });

    if let Some(asset) = asset {
      tracing::trace!("Found existing asset");
      return Ok(asset)
    }

    tracing::trace!("Initializing new asset");
    let asset : ActiveModel = ActiveModel {
      id: Set(id),
      ..Default::default()
    }.insert(&self.db)
     .await?
     .into();

    Ok(asset)
  }

  pub async fn consume(&self, event: TradingViewEvent) -> Result<Option<Model>> {
    let asset = match event.clone() {
      TradingViewEvent::Title(ticker, title) => {
        let mut asset = self.find_asset(ticker).await?;

        asset.name = Set(Some(title));
        Some(asset)
      },
      TradingViewEvent::Type(ticker, kind) => {
        let mut asset = self.find_asset(ticker).await?;

        asset.kind = Set(Some(kind));
        Some(asset)
      },
      TradingViewEvent::Description(ticker, description) => {
        let mut asset = self.find_asset(ticker).await?;

        asset.description = Set(Some(description));
        Some(asset)
      },
      TradingViewEvent::WebsiteUrl(ticker, website_url) => {
        let mut asset = self.find_asset(ticker).await?;

        asset.website_url = Set(Some(website_url));
        Some(asset)
      },
      TradingViewEvent::Isin(ticker, isin) => {
        let mut asset = self.find_asset(ticker).await?;

        asset.isin = Set(Some(isin));
        Some(asset)
      },
      TradingViewEvent::CurrencyCode(ticker, currency) => {
        let mut asset = self.find_asset(ticker).await?;

        asset.currency = Set(Some(currency));
        Some(asset)
      },
      TradingViewEvent::Logo(ticker, logo_url) => {
        let mut asset = self.find_asset(ticker).await?;

        asset.logo_url = Set(Some(logo_url));
        Some(asset)
      },
      TradingViewEvent::BaseCurrencyLogo(ticker, logo_url) => {
        let mut asset = self.find_asset(ticker).await?;

        asset.logo_url = Set(Some(logo_url));
        Some(asset)
      },
      TradingViewEvent::CurrencyLogo(ticker, currency_logo_url) => {
        let mut asset = self.find_asset(ticker).await?;

        asset.currency_logo_url = Set(Some(currency_logo_url));
        Some(asset)
      },
      TradingViewEvent::Exchange(ticker, exchange) => {
        let mut asset = self.find_asset(ticker).await?;

        asset.exchange = Set(Some(exchange));
        Some(asset)
      },
      TradingViewEvent::Country(ticker, country) => {
        let mut asset = self.find_asset(ticker).await?;

        asset.country = Set(Some(country));
        Some(asset)
      },
      _ => None
    };

    if let Some(asset) = asset {
      tracing::trace!("Updating asset with event: {:?}", event);
      let asset = asset.update(&self.db).await?;
      return Ok(Some(asset));
    }

    Ok(None)
  }
}

#[cfg(test)]
mod test {
  use anyhow::Context;
  //use entity::{prelude::*, EntityTrait};
  use trading_view::{TradingViewEvent, Ticker};
  use crate::test::db;
  use super::*;

  async fn consumer() -> Result<(DatabaseConnection, TradingViewEventConsumer)> {
    let db = db().await?;
    return Ok((db.clone(), TradingViewEventConsumer::new(db.clone())));
  }

  #[tokio::test]
  async fn consume_title() -> Result<()> {
    let (_db, consumer) = consumer().await?;
    let asset = consumer.consume(
      TradingViewEvent::Title(
        Ticker::new(Some("NASDAQ"), "TSLA"),
        "Hello world".to_owned()
      )
    )
    .await
    .with_context(|| "Could not consume event")?
    .unwrap();

    assert_eq!(asset.name, Some("Hello world".to_owned()));
    assert_eq!(asset.id, "NASDAQ:TSLA");
    Ok(())
  }

  #[tokio::test]
  async fn update_existing_asset() -> Result<()> {
    let (db, consumer) = consumer().await?;
    ActiveModel {
      id: Set("USDPLN".to_owned()),
      ..Default::default()
    }.insert(&db).await?;

    let asset = consumer.consume(
      TradingViewEvent::Title(
        Ticker::new(None, "USDPLN"),
        "Hello world2".to_owned()
      )
    )
    .await
    .with_context(|| "Could not consume event")?
    .unwrap();

    assert_eq!(asset.name, Some("Hello world2".to_owned()));
    assert_eq!(asset.id, "USDPLN");
    Ok(())
  }
}
