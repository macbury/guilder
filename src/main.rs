extern crate lazy_static;
#[macro_use] extern crate rocket;
extern crate dotenv;

use dotenv::dotenv;

#[cfg(test)]
#[macro_use]
mod test;
#[cfg(test)]
mod specs;
mod types;
mod actions;
mod env;
mod stages;
mod services;
mod cli;
mod utils;
mod secure;

use clap::Parser;
use cli::{GuilderCli, Commands};
use tracing_subscriber::{EnvFilter};

fn setup_tracing() {
  use tracing_subscriber::fmt::writer::MakeWriterExt;
  let stdout = std::io::stdout
    .with_max_level(tracing::Level::DEBUG);
  let log_appender = tracing_appender::rolling::never("./logs", "app.log");
  tracing_subscriber::fmt()
    .compact()
    .with_env_filter(EnvFilter::from_default_env())
    .with_writer(stdout.and(log_appender))
    //.with_file(true)
    .without_time()
    //.with_span_events(tracing_subscriber::fmt::format::FmtSpan::FmtSpan::CLOSE)
    .with_thread_ids(true)
    //.with_test_writer()
    //.with_max_level(tracing::Level::DEBUG)
    .init();
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
  dotenv().ok();
  setup_tracing();

  let config = env::config();
  let cli = GuilderCli::parse();
  if let Some(action) = cli.action {
    match action {
      Commands::Register { username, password  } => {
        cli::commands::create_user(&username, &password, &config)
          .await
          .expect("Could not create user...");
        return Ok(());
      },

      _ => {}
    }
  }

  let _rocket = env::server(config)
    .attach(stages::jobs::liftoff())
    .launch()
    .await?;

  Ok(())
}
