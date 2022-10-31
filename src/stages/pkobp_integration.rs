use entity::BondPerformance;
use rocket::{fairing::{AdHoc, self}, Rocket, Build};
use sea_orm_rocket::Database;
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio_cron_scheduler::{JobScheduler, Job};

use crate::{services::bonds::pkobp::PKOBondsImporter, env::GuilderConfig};
use super::{Db, events::EventQueue};

/**
 * Lockable PKOBondsImporter
 */
pub type StatePKOBondsImporter = Arc<RwLock<PKOBondsImporter>>;

/**
 * Run importer sync
 */
#[tracing::instrument(name = "job_sync", skip(importer))]
async fn sync(importer : StatePKOBondsImporter) -> anyhow::Result<()> {
  tracing::debug!("Syncing bonds in background, locking importer");
  // Ensure that only one importer can run at once!
  let importer = importer.write().await;

  tracing::debug!("Importer locked, starting...");
  importer.sync_all().await?;
  tracing::debug!("Finished syncing, bye bye");
  Ok(())
}

pub fn stage() -> AdHoc {
  AdHoc::on_ignite("Configure PKOBP Integration", |rocket| async move {
    let event_queue = rocket.state::<EventQueue>()
      .expect("Missing event queue")
      .clone();
    let db = Db::fetch(&rocket)
      .expect("Missing database connection");
    let config = rocket.state::<GuilderConfig>()
      .expect("Missing guilder config state");
    let importer_conn = db.conn.clone();
    let refresh_conn = db.conn.clone();
    let importer = Arc::new(
      RwLock::new(
        PKOBondsImporter::new(
          importer_conn,
          config.selenium_hub_url.clone(),
          config.encryption.clone(),
          event_queue
        )
      )
    );

    // Store importer in rocket state
    let rocket = rocket
      .manage(importer.clone());

    // Configure scheduler to run importer every 2 hours
    let scheduler = rocket.state::<JobScheduler>()
      .expect("Missing scheduler, check if stage() was run before");

    let refresh_bonds_job = Job::new_async("0 0 1,6,8,10,12,14,16,18 * * *", move |_uuid, _l| {
      let importer = importer.clone();
      Box::pin(async move {
        if let Err(error) = sync(importer).await {
          tracing::error!("Failed refreshing bonds: {:?}", error);
        }
      })
    }).expect("Could not initialize job for refreshing pkobp");

    scheduler.add(refresh_bonds_job)
      .expect("Could not register refresh bond job");

    let refresh_views = Job::new_async("0 0 0,2,3,4 * * *", move |_uuid, _l| {
      let refresh_conn = refresh_conn.clone();
      Box::pin(async move {
        if let Err(error) = BondPerformance::refresh(&refresh_conn).await {
          tracing::error!("Failed refreshing bonds: {:?}", error);
        }
      })
    }).expect("Could not initialize job for refresh_views pkobp");

    scheduler.add(refresh_views)
      .expect("Could not register refresh bond job");

    rocket
  })
}

pub async fn refresh_views(rocket: Rocket<Build>) -> fairing::Result {
  let conn = &Db::fetch(&rocket)
    .expect("Could not run view refresh. Is database running?")
    .conn;
  if let Err(error) = BondPerformance::refresh(&conn).await {
    tracing::error!("Migration failed: {:?}", error);
    Err(rocket)
  } else {
    Ok(rocket)
  }
}
