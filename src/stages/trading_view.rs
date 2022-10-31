//TODO: Holy fuck, this is fucking mess, try to reimplement this similar way as which bonds, and tokio scheduler
use std::{borrow::Cow, time::Duration};

use entity::{Asset, EntityTrait};
use rocket::{fairing::AdHoc, request::FromParam};
use tokio::sync::broadcast::error::SendError;
use trading_view::{AssetsManager, Ticker, TradingViewEvent};
use sea_orm_rocket::Database;
use crate::{services::{TradingViewEventConsumer, TradingViewDataPointsConsumer}, stages::events::ServerEvent};

use super::{Db, events::EventQueue};

#[derive(Debug)]
pub struct TickerParam<'a>(Cow<'a, Ticker>);

impl<'a> FromParam<'a> for TickerParam<'a> {
  type Error = &'a str;

  fn from_param(param: &'a str) -> Result<Self, Self::Error> {
    let ticker : Result<Ticker, _> = param.try_into();
    match ticker {
      Ok(ticker) => {
        Ok(
          Self(Cow::Owned(ticker))
        )
      },
      Err(_err) => Err("Invalid ticker")
    }
  }
}

impl Into<Ticker> for TickerParam<'_> {
  fn into(self) -> Ticker {
    self.0.into_owned()
  }
}

async fn process_event(event : &TradingViewEvent, event_consumer: &TradingViewEventConsumer, event_queue: &EventQueue, points_consumer : &mut TradingViewDataPointsConsumer) -> anyhow::Result<()> {
  let event_queue = event_queue.clone();

  match event_consumer.consume(event.clone()).await {
    Err(error) => {
      tracing::error!("Could not consume: {:?} because of {:?}", event, error);
      return Err(error)
    },
    Ok(asset) if asset.is_some() => {
      let asset = asset.unwrap();
      event_queue.send(ServerEvent::AssetMetadataUpdated(asset.ticker()))?;
      return Ok(())
    },
    _ => {},
  }

  match points_consumer.consume(event.clone()).await {
    Err(error) => {
      tracing::error!("Could not consume: {:?} because of {:?}", event, error);
      return Err(error)
    },
    Ok(ticker) if ticker.is_some() => {
      //event_queue.send(ServerEvent::AssetMetadataUpdated(ticker.unwrap()))?;
      return Ok(())
    },
    _ => {},
  }

  tracing::trace!("Skipping event");
  Ok(())
}

async fn flush_events(points_consumer : &mut TradingViewDataPointsConsumer, event_queue: &EventQueue) -> anyhow::Result<()> {
  let tickers = points_consumer.flush().await?;

  if let Some(tickers) = tickers {
    tracing::debug!("Flushing tickers: {:?}", tickers);
    event_queue.send(ServerEvent::UpdatedPoints(tickers))?;
  }

  Ok(())
}

fn this_is_not_a_send_failure(error: &anyhow::Error) -> bool {
  error.downcast_ref::<SendError<ServerEvent>>().is_none()
}

pub fn stage() -> AdHoc {
  let (asset_manager, mut stream) = AssetsManager::new();

  AdHoc::on_ignite("TradingView", |rocket| async {
    let conn = &Db::fetch(&rocket)
      .expect("Missing database connection").conn;
    let mut points_consumer = TradingViewDataPointsConsumer::new(conn.clone());
    let event_consumer = TradingViewEventConsumer::new(conn.clone());
    let event_queue = rocket.state::<EventQueue>()
      .expect("Missing event queue")
      .clone();

    tokio::spawn(async move {
      loop {
        if let Err(err) = flush_events(&mut points_consumer, &event_queue).await {
          if this_is_not_a_send_failure(&err) {
            tracing::error!("Could not flush tickers: {:?}", err);
          } else {
            tracing::trace!("Could not send tickers: {:?}", err);
          }
        }

        let timeout_sleep = tokio::time::sleep(Duration::from_secs(10));
        tokio::pin!(timeout_sleep);

        tracing::trace!("Waiting for event...");
        let event = tokio::select! {
          v = stream.recv() => v,
          _ = &mut timeout_sleep => {
            tracing::trace!("timeout, waiting for new events from trading view...");
            None
          }
        };

        if let Some(event) = event {
          if let Err(err) = process_event(&event, &event_consumer, &event_queue, &mut points_consumer).await {
            if this_is_not_a_send_failure(&err) {
              tracing::error!("Could not process event: {:?}", err);
            } else {
              tracing::trace!("Could not send event: {:?}", err);
            }
          }
        } else {
          tracing::trace!("No event, waiting 1 second...");
          tokio::time::sleep(Duration::from_secs(1)).await;
        }
      }
    });

    return rocket
      .manage(asset_manager)
      .attach(AdHoc::try_on_ignite("Observe assets", |rocket| async {
        let mut asset_manager = rocket.state::<AssetsManager>().unwrap().clone();
        let db = &Db::fetch(&rocket).unwrap().conn;
        let assets = Asset::find().all(db).await
          .expect("Could not fetch list of assets");

        for asset in assets {
          tracing::info!("Observing asset: {:?}", asset.id);
          let ticker : Ticker = asset.id.try_into()
            .expect("Invalid ticker type in database!");

          asset_manager.observe(ticker)
            .await
            .expect("Could not observe ticker");
        }
        Ok(rocket)
      }));
  })
}
