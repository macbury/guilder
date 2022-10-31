use rocket::{fairing::{AdHoc, self}, Rocket, Build};
use sea_orm_rocket::Database;
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio_cron_scheduler::{JobScheduler, Job};
use trading_view::AssetsManager;

use crate::services::exchange_rates::ExchangeRates;
use super::Db;

/**
 * Lockable PKOBondsImporter
 */
pub type StateExchangeRates = Arc<RwLock<ExchangeRates>>;

#[tracing::instrument(name = "exchange_rate_sync_job", skip(exchange_rates))]
async fn sync(exchange_rates : &StateExchangeRates) -> anyhow::Result<()> {
  tracing::debug!("Syncing exchange rates waiting for lock...");
  let mut exchange_rates = exchange_rates.write().await;
  tracing::debug!("Lock ready starting sync");
  exchange_rates.sync().await?;
  tracing::debug!("Finished sync");
  Ok(())
}

pub fn stage() -> AdHoc {
  AdHoc::on_ignite("Exchange Rate", |rocket| async {
    let conn = &Db::fetch(&rocket)
      .expect("Could not run migrations. Is database running?")
      .conn;
    let assets_manager = rocket.state::<AssetsManager>()
      .expect("Missing assets manager")
      .clone();

    let exchange_rates = Arc::new(
      RwLock::new(
        ExchangeRates::new(&assets_manager, &conn)
      )
    );

    let rocket = rocket
      .manage(exchange_rates.clone());

    let scheduler = rocket.state::<JobScheduler>()
      .expect("Missing scheduler, check if stage() was run before");

    let exchange_rates_job = Job::new_async("0 */15 * * * *", move |_uuid, _l| {
      let exchange_rates = exchange_rates.clone();
      Box::pin(async move {
        if let Err(error) = sync(&exchange_rates).await {
          tracing::error!("Failed syncing exchange rates: {:?}", error);
        }
      })
    }).expect("Could not initialize job for refreshing exchange rates");

    scheduler.add(exchange_rates_job)
      .expect("Could not register refresh exchange rates job");

    rocket
  })
}

pub async fn sync_exchange_rates(rocket: Rocket<Build>) -> fairing::Result {
  let exchange_rates = rocket.state::<StateExchangeRates>().unwrap().clone();
  if let Err(error) = sync(&exchange_rates).await {
    tracing::error!("Syncing exchange rates failed: {:?}", error);
    Err(rocket)
  } else {
    Ok(rocket)
  }
}
