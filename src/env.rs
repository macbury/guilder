use std::collections::HashMap;
use std::env;

use entity::sea_orm::DbErr;
use rocket::Rocket;
use rocket::serde::json::Json;
use rocket::{figment::Figment, Build, fairing::AdHoc};
use serde::{Serialize, Deserialize};
use sea_orm_rocket::Database;
use validator::{Validate, ValidationErrors};

use crate::stages;
use crate::actions;

#[derive(Debug, Deserialize, Serialize)]
pub struct GuilderConfig {
  pub salt: String,
  pub selenium_hub_url: String,
  pub encryption: Vec<u8>,
}

impl Default for GuilderConfig {
  fn default() -> Self {
    Self { salt: "".to_owned(), encryption: vec![], selenium_hub_url: "".to_owned() }
  }
}

pub type ResponseResult<T = ()> = anyhow::Result<T, rocket::response::Debug<anyhow::Error>>;

#[derive(Serialize, Debug)]
pub struct CrudResult<T> {
  success: bool,
  resource: Option<T>,
  errors: Option<ValidationErrorsMessages>
}

pub type CrudResponse<M> = ResponseResult<Option<Json<CrudResult<M>>>>;

impl<T> CrudResult<T> {
  pub fn success(resource : T) -> ResponseResult<Option<Json<Self>>> {
    Ok(
      Some(
        Json(
          Self {
            success: true,
            resource: Some(resource),
            errors: None
          }
        )
      )
    )
  }

  pub fn not_found() -> ResponseResult<Option<Json<Self>>> {
    Ok(None)
  }

  pub fn fail(errors : ValidationErrorsMessages) -> ResponseResult<Option<Json<Self>>> {
    Ok(
      Some(
        Json(
          Self {
            success: false,
            resource: None,
            errors: Some(errors)
          }
        )
      )
    )
  }
}

pub fn catch_error(e : DbErr) -> anyhow::Error {
  anyhow::anyhow!("Could not find record: {}", e)
}

pub type ValidationErrorsMessages = HashMap<String, Vec<String>>;

/**
 * Validate object and return hash string with errors for json consumption
 */
pub fn validate(data : &impl Validate) -> Result<(), (ValidationErrors, ValidationErrorsMessages)> {
  if let Err(err) = data.validate() {
    tracing::error!("Validation failed: {:?}", err);
    let errors: HashMap<String, Vec<String>> = err.field_errors().iter().map(|(field, errors)| {
      let messages : Vec<String> = errors.iter().map(|a| {
        let msg = a.clone().message.expect("Missing validation error message").into_owned();
        msg
      }).collect();
      return (field.to_string(), messages)
    }).collect();

    return Err((err, errors));
  }

  Ok(())
}

pub fn config() -> Figment {
  let secret = env::var("SECRET_KEY").expect("Missing SECRET_KEY");
  let encryption = env::var("ENCRYPTION_KEY").expect("Missing ENCRYPTION_KEY");
  let selenium_hub_url = env::var("SELENIUM_HUB_URL").expect("Missing SELENIUM_HUB_URL");
  let config = rocket::Config::figment()
    .merge(("databases.guilder.url", env::var("DATABASE_URL").expect("Missing DATABASE_URL")))
    .merge(("secret_key", secret.clone()))
    .merge(("salt", secret))
    .merge(("address", "0.0.0.0"))
    .merge(("encryption", encryption.as_bytes()))
    .merge(("selenium_hub_url", selenium_hub_url));

  let max_workers = config.extract_inner::<i64>("workers")
    .expect("Could not load number of workers from config");

  config
    .merge(("databases.guilder.max_connections", max_workers + 4)) //TODO: better way of calculating how much more connections we need
}

pub fn server(config : Figment) -> Rocket<Build> {
  rocket::custom(config)
    .attach(stages::events::stage())
    .attach(AdHoc::config::<GuilderConfig>())
    .attach(stages::Db::init())
    .attach(AdHoc::try_on_ignite("Migrations", stages::db::run_migrations))
    .attach(stages::jobs::stage())
    .attach(stages::trading_view::stage())
    .attach(stages::pkobp_integration::stage())
    .attach(stages::exchange_rate::stage())
    .attach(AdHoc::try_on_ignite("PKOBP view refresh", stages::pkobp_integration::refresh_views))
    .attach(AdHoc::try_on_ignite("Exchange Rates Sync", stages::exchange_rate::sync_exchange_rates))
    .attach(actions::api::stage())
    .attach(actions::root::stage())
}
