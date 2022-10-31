use anyhow::{Result, Context};
use tokio_cron_scheduler::JobScheduler;
use rocket::fairing::AdHoc;

async fn configure_scheduler() -> Result<JobScheduler> {
  let sched = JobScheduler::new()?;
  Ok(sched)
}

/**
 * On each stage use rocket.state::<JobScheduler>() to get scheduler and add job to run:
 * let job = Job::new("1/10 * * * * *", |uuid, _l| {
 *  //https://crates.io/crates/tokio-cron-scheduler
 *   tracing::info!("I run every 10 seconds: {:?}", uuid);
 * })?;
 * sched.add(job)?;
 */
pub fn stage() -> AdHoc {
  AdHoc::on_ignite("Configure Jobs Scheduler", |rocket| async {
    let sched = configure_scheduler()
      .await
      .with_context(|| "Could not configure jobs")
      .unwrap();

    return rocket
      .manage(sched);
  })
}

pub fn liftoff() -> AdHoc {
  AdHoc::on_liftoff("Starting Jobs Scheduler", |rocket| Box::pin(async move {
    let scheduler = rocket.state::<JobScheduler>()
      .expect("Missing scheduler, check if stage() was run before");

    scheduler.start()
      .expect("Could not start jobs scheduler");
  }))
}
