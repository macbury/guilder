use crate::env as app_env;
use crate::services::NewUser;
use std::env;
use std::sync::atomic::{AtomicUsize, Ordering};
use dotenv;
use anyhow::Result;
use entity::{ConnectionTrait, Statement};
use entity::sea_orm::{Database, DatabaseConnection, DatabaseBackend};
use rocket::http::{ContentType, Cookie};
use rocket::local::asynchronous::{Client, LocalResponse};
use url::Url;
use migration::MigratorTrait;
use serde_json::Value;

static DATABASE_UUID: AtomicUsize = AtomicUsize::new(0);

#[macro_export(local_inner_macros)]
macro_rules! json {
  ($($json:tt)+) => {{
    let value = serde_json::json!($($json)+);
    serde_json::to_string(&value).unwrap()
  }};
}

#[macro_export(local_inner_macros)]
macro_rules! json_response {
  ($($json:tt)+) => {{
    serde_json::json!($($json)+)
  }};
}

pub async fn compare_body<'a>(response : LocalResponse<'a>, expected : Value) {
  let current_response : serde_json::Value = serde_json::from_str(&response.into_string().await.unwrap()).unwrap();
  assert_eq!(current_response, expected);
}

#[macro_export(local_inner_macros)]
macro_rules! assert_body {
  ($a:expr, $($json:tt)+) => {{
    let value = serde_json::json!($($json)+);
    crate::test::compare_body($a, value)
  }};
}

async fn db_and_url() -> Result<(DatabaseConnection, String)> {
  dotenv::from_filename(".env.test").ok();
  // tracing_subscriber::fmt()
  //   .with_test_writer()
  //   .init();
  env::set_var("DATABASE_URL", "boom"); // set invalid db, this should throw error in other places
  let uuid = DATABASE_UUID.fetch_add(1, Ordering::SeqCst);
  let master_db_url = env::var("MASTER_DATABASE_URL").expect("Missing MASTER_DATABASE_URL");

  let master_db: DatabaseConnection = Database::connect(master_db_url.clone()).await?;
  let temp_db_name = format!("guilder_test_{}", uuid);

  master_db.query_one(Statement::from_string(
    DatabaseBackend::Postgres,
    format!("DROP DATABASE IF EXISTS {}", temp_db_name).to_owned(),
  )).await?;

  master_db.query_one(Statement::from_string(
    DatabaseBackend::Postgres,
    format!("CREATE DATABASE {}", temp_db_name).to_owned(),
  )).await?;

  let mut db_url = Url::parse(&master_db_url).expect("Invalid database url");
  db_url.set_path(&temp_db_name);
  tracing::info!("Using url: {}", db_url);

  let db_url = db_url.as_str().to_string();
  let db: DatabaseConnection = Database::connect(&db_url).await?;
  migration::Migrator::up(&db, None).await?;
  Ok((db, db_url))
}

pub async fn db() -> Result<DatabaseConnection> {
  let (db, _) = db_and_url().await?;
  Ok(db)
}

pub async fn server() -> (Client, DatabaseConnection) {
  let (db, url) = db_and_url().await
    .expect("Could not boot up db");
  let config = app_env::config()
    .merge(("databases.guilder.max_connections", 4i32))
    .merge(("databases.guilder.url", url.to_string()));

  let app = app_env::server(config);

  let client = Client::tracked(app).await
    .expect("valid rocket instance");

  (client, db)
}

pub async fn create_user(login: &str, password: &str, db: &DatabaseConnection) -> entity::user::Model {
  let secret = env::var("SECRET_KEY").expect("Missing SECRET_KEY");
  let new_user = NewUser::new(login, password, &secret);
  let user = new_user.save(db).await.expect("Could not create user for test");

  return user;
}

pub async fn sign_in<'a>(client: &Client, db: &DatabaseConnection) -> (entity::user::Model, Cookie<'a>) {
  let user = create_user("admin", "admin1234", db).await;

  let response = client.post("/api/sign_in")
    .header(ContentType::JSON)
    .body(json!({
      "login": "admin",
      "password": "admin1234"
    }))
    .dispatch()
    .await;

  let cookie = response.cookies()
    .get_private("user_id")
    .expect("Missing cookie from sign in")
    .to_owned();

  (user, cookie)
}
