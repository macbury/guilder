use async_trait::async_trait;
use entity::sea_orm;
use rocket::{Rocket, Build, fairing};
use sea_orm::ConnectOptions;
use sea_orm_rocket::{rocket::figment::Figment, Config, Database};
use std::time::Duration;
use migration::{Migrator, MigratorTrait};

#[derive(Database, Debug)]
#[database("guilder")]
pub struct Db(SeaOrmPool);

#[derive(Debug, Clone)]
pub struct SeaOrmPool {
  pub conn: sea_orm::DatabaseConnection,
}

#[async_trait]
impl sea_orm_rocket::Pool for SeaOrmPool {
  type Error = sea_orm::DbErr;

  type Connection = sea_orm::DatabaseConnection;

  async fn init(figment: &Figment) -> Result<Self, Self::Error> {
    let config = figment.extract::<Config>().expect("Missing configuration...");
    let mut options: ConnectOptions = config.url.into();
    tracing::info!("Max database connections: {}", config.max_connections);
    //tracing::info!("Figment: {:?}", figment);
    options
      .sqlx_logging(true)
      .max_connections(config.max_connections as u32)
      .min_connections(config.min_connections.unwrap_or(1))
      .connect_timeout(Duration::from_secs(config.connect_timeout));

    if let Some(idle_timeout) = config.idle_timeout {
      options.idle_timeout(Duration::from_secs(idle_timeout));
    }

    let conn = sea_orm::Database::connect(options).await?;

    Ok(SeaOrmPool { conn })
  }

  fn borrow(&self) -> &Self::Connection {
    &self.conn
  }
}

pub async fn run_migrations(rocket: Rocket<Build>) -> fairing::Result {
  let conn = &Db::fetch(&rocket)
    .expect("Could not run migrations. Is database running?")
    .conn;
  if let Err(error) = Migrator::up(conn, None).await {
    tracing::error!("Migration failed: {:?}", error);
    Err(rocket)
  } else {
    Ok(rocket)
  }
}
