use anyhow::{Result, Context, Ok};
use chrono::Utc;
use entity::{
  sea_orm::DatabaseConnection,
  Integration,
  EntityTrait,
  IntegrationModel,
  integrations::{Status, ActiveModel},
  Set,
  ActiveModelTrait,
  TransactionTrait,
  Bond,
  bonds::{self, BondStatus}, import::ImportDataPoints,
  points, BondPerformance
};
use pkobp::{PKOBPAccount, PKOBPBond};
use rust_decimal::prelude::ToPrimitive;
use rusty_money::iso;

use crate::{
  secure,
  stages::events::{
    EventQueue, ServerEvent
  },
  utils::{now, self}
};

/**
 * Synchronize bonds from PKOBP account into Guilder bonds
 */
pub struct PKOBondsImporter {
  db: DatabaseConnection,
  selenium_hub_url: String,
  encryption_key: Vec<u8>,
  event_queue: EventQueue
}

impl PKOBondsImporter {
  pub fn new(db : DatabaseConnection, selenium_hub_url: String, encryption_key: Vec<u8>, event_queue: EventQueue) -> Self {
    Self { db, selenium_hub_url, encryption_key, event_queue }
  }

  /**
   * Push integration model data to client, this will update the ui.
   */
  fn push_event(&self, integration : &IntegrationModel) {
    if let Err(error) = self.event_queue.send(ServerEvent::UpdatedIntegration(integration.id)) {
      tracing::warn!("Could not push integration update: {:?}", error);
    }
  }

  async fn refresh_views(&self) -> Result<()> {
    BondPerformance::refresh(&self.db).await
  }

  /**
   * Simple helper method to change status of integration, Maybe I should move this to Entity itself?
   */
  async fn change_status(&self, integration : &IntegrationModel, status : Status, update_time : bool) -> Result<()> {
    let mut model : ActiveModel = integration.clone().into();
    model.status = Set(status);

    if update_time {
      model.last_sync_at = Set(Some(Utc::now()));
    }

    let integration = model.update(&self.db)
      .await
      .with_context(|| "Could not persist status change for model")?;
    self.push_event(&integration);

    Ok(())
  }

  /**
   * Find all integrations with kind PKOBP, and sync every one
   */
  #[tracing::instrument(name="syncAll", level="debug", skip(self))]
  pub async fn sync_all(&self) -> Result<()> {
    let integrations = Integration::find()
      .all(&self.db)
      .await?;

    if integrations.len() == 0 {
      tracing::debug!("No integrations found, skipping...");
      return Ok(());
    }

    tracing::debug!("Found {} integrations, syncing...", integrations.len());

    for integration in integrations {
      self.sync(&integration, false).await?;
    }

    self.refresh_views().await?;
    Ok(())
  }

  /**
   * Sync only one integration
   */
  #[tracing::instrument(name="sync", level="trace", skip(self, integration))]
  pub async fn sync(&self, integration : &IntegrationModel, regenerate_view: bool) -> Result<()> {
    self.change_status(integration, Status::Syncing, false).await?;

    if let Err(error) = self.fetch_bonds_and_persist(integration).await {
      tracing::warn!("Could not fetch bonds: {:?}", error);
      self.change_status(integration, Status::Error, false).await?;
      Err(error)
    } else {
      if regenerate_view {
        self.refresh_views().await?;
      }
      self.change_status(integration, Status::Done, true).await?;
      Ok(())
    }
  }

  /**
   * Separate function which syncs fetch bonds using selenium from PKOBP and then persist them into database,
   * Everything is run in transaction, which ensures that nothing funky happens
   */
  async fn fetch_bonds_and_persist(&self, integration : &IntegrationModel) -> Result<()> {
    let encrypted_password = integration.password.as_deref();//TODO skip if password is missing...
    let password = secure::decrypt_text(encrypted_password.unwrap(), &self.encryption_key)?;
    let mut account = PKOBPAccount::new(&self.selenium_hub_url, &integration.login, &password);

    account.sync().await?;

    tracing::trace!("Number of bonds in PKOBPAccount: {}", account.bonds.len());
    tracing::trace!("Current cash: {}", account.cash); //TODO: this should be tracked

    let txn = self.db.begin().await?;
    let mut existing_bonds_id : Vec<i64> = Vec::new();
    for pko_bond in account.bonds {
      let bond = self.create_bond(integration.id, &pko_bond).await
        .with_context(|| "Could not import bond")?;
      existing_bonds_id.push(bond.id.unwrap());
    }

    tracing::debug!("Created/Updated bonds: {:?}", existing_bonds_id);
    self.create_dividends(integration).await?;
    self.archive_bonds(existing_bonds_id, integration).await?;

    txn.commit().await?;
    Ok(())
  }

  #[tracing::instrument(name="archive_bonds", level="trace", skip(self))]
  async fn create_dividends(&self,integration : &IntegrationModel) -> Result<()> {
    tracing::error!("Missing functionality....");
    //TODO: all interest calculation should happen here by fetching All Active bonds and then creating dividends from interests before trying to archive them
    //TODO: this should prevent situation that if system is down for long time, we lose all old dividends
    Ok(())
  }

  #[tracing::instrument(name="archive_bonds", level="trace", skip(self))]
  async fn archive_bonds(&self, existing_bonds_ids: Vec<i64>, integration : &IntegrationModel) -> Result<()> {
    let result = Bond::update_candidates_as_archived(existing_bonds_ids, integration.id)
      .exec(&self.db)
      .await
      .with_context(|| "Failed archiving bonds...")?;
    tracing::debug!("Marked {} as archived", result.rows_affected);

    Ok(())
  }

  /**
   * Find or create single bond
   */
  #[tracing::instrument(name="create_bond", level="trace", skip(self, pko_bond))]
  async fn create_bond<'a>(&self, integration_id : i64, pko_bond : &'a PKOBPBond<'a>) -> Result<bonds::ActiveModel> {
    let mut data_points = ImportDataPoints::new(&self.db, "Bond");

    let uid = pko_bond.id();
    tracing::trace!("Checking bond: {}", uid);

    let interest_periods = pko_bond.all_interests();
    let current_period = interest_periods.last().expect("Missing periods, something is wrong...");

    let mut bond = Bond::find_or_initialize_by_uid(&uid, integration_id, &self.db).await?;
    bond.emission = Set(pko_bond.emission.clone());
    bond.start_date = Set(pko_bond.start_date());
    bond.end_date = Set(pko_bond.buyout_date);
    bond.interest_date = Set(current_period.end_date);
    bond.name = Set(pko_bond.kind.name().to_owned());
    let kind : String = pko_bond.kind.into();
    bond.kind = Set(format!("PL:{}", kind));
    bond.updated_at = Set(now());
    bond.status = Set(BondStatus::Active);
    bond.currency = Set(iso::PLN.iso_alpha_code.to_owned());
    let period_start : Vec<String> = interest_periods.iter().map(|p| p.start_date.format("%Y-%m-%d").to_string()).collect();
    let period_end : Vec<String> = interest_periods.iter().map(|p| p.end_date.format("%Y-%m-%d").to_string()).collect();

    bond.period_start = Set(serde_json::to_value(period_start)?);
    bond.period_end = Set(serde_json::to_value(period_end)?);

    let bond = bond
      .save(&self.db)
      .await
      .with_context(|| "Could not persist bond")?;

    let bond_id = bond.clone().id.unwrap().to_string();
    let today = utils::today();
    tracing::trace!("Updated/Created bond: {:?}", bond_id);

    data_points.change_overwrite(false).await?;

    let historic_values = pko_bond.past_interests_to(&utils::today());
    tracing::trace!("There is {} historic values to insert", historic_values.len());
    for (date, balance, rate, index) in historic_values.iter() {
      let early_buyout_price = pko_bond.early_buyout_price(*index, &balance);
      data_points.add_option(bond_id.clone(), points::Kind::BuyoutPrice, *date, early_buyout_price.amount().to_f64()).await?;
      data_points.add(bond_id.clone(), points::Kind::Period, *date, index.to_f64().unwrap()).await?;
      data_points.add_option(bond_id.clone(), points::Kind::StartPrice, *date, pko_bond.start_price.amount().to_f64()).await?;
      data_points.add(bond_id.clone(), points::Kind::Rate, *date, *rate).await?;
      data_points.add_option(bond_id.clone(), points::Kind::Price, *date, balance.amount().to_f64()).await?;
    }

    tracing::trace!("Inserting fresh data");
    data_points.change_overwrite(true).await?;

    // let rate = pko_bond.rates.last().map(|v| *v);
    data_points.add(bond_id.clone(), points::Kind::Volume, today, pko_bond.shares.into()).await?;
    data_points.add_option(bond_id.clone(), points::Kind::StartPrice, today, pko_bond.start_price.amount().to_f64()).await?;
    // data_points.add_option(bond_id.clone(), points::Kind::Rate, today, rate).await?; //pkobp has broken rate reporting, use our simulation for it

    if !pko_bond.simulate_interest() {
      let current_price = pko_bond.current_price.amount().to_f64().unwrap_or_default();
      data_points.add(bond_id.clone(), points::Kind::Price, today, current_price).await?;
    }

    data_points.commit().await?;

    Ok(bond)
  }
}

#[cfg(test)]
mod test {
  use rusty_money::{Money, iso};
  use chrono::NaiveDate;
  use entity::{integrations::{self, Kind, ActiveModel}, ActiveModelBehavior, Point};
  use pkobp::BondKind;
  use rocket::tokio::sync::broadcast::channel;
  use crate::test::db;
  use super::*;

  async fn prepare_importer() -> Result<(PKOBondsImporter, ActiveModel, DatabaseConnection)> {
    let (sender, _receiver) = channel::<ServerEvent>(1024);
    let db = db().await?;
    let key : Vec<u8> = vec![];
    let mut integration = integrations::ActiveModel::new();
    integration.name = Set("Testing".to_owned());
    integration.login = Set("Login".to_owned());
    integration.kind = Set(Kind::PKOBP);
    let integration = integration.save(&db).await?;

    let importer = PKOBondsImporter::new(
      db.clone(), "http://localhost:4444".to_owned(), key, sender
    );

    Ok((importer, integration, db))
  }

  #[tokio::test]
  async fn create_new_bond() -> Result<()> {
    let (importer, integration, db) = prepare_importer().await?;

    let pko_bond = PKOBPBond {
      kind: BondKind::COI,
      emission: "COI123".to_owned(),
      shares: 10,
      start_price: Money::from_minor(1000, iso::PLN),
      current_price: Money::from_minor(1010, iso::PLN),
      rates: vec![1.1, 4.3],
      buyout_date: NaiveDate::from_ymd(2012, 1, 1),
      ..Default::default()
    };

    let model = importer.create_bond(integration.id.unwrap(), &pko_bond).await?;
    assert_eq!(model.kind.unwrap(), "PL:COI");
    assert_eq!(model.emission.unwrap(), "COI123");
    assert_eq!(model.name.unwrap(), "Obligacje 4-letnie COI");

    let points = Point::find().all(&db).await?;
    assert_eq!(points.len(), 3658);
    Ok(())
  }

  #[tokio::test]
  async fn test_builded_dor_bond() -> Result<()> {
    let (importer, integration, _db) = prepare_importer().await?;

    let pko_bond = PKOBPBond {
      kind: BondKind::ROR,
      emission: "ROR0623".to_owned(),
      shares: 20,
      start_price: Money::from_major(2000, iso::PLN),
      current_price: Money::from_minor(2007, iso::PLN),
      rates: vec![5.25, 6.0],
      buyout_date: NaiveDate::from_ymd(2023, 6, 1),
      ..Default::default()
    };

    let model = importer.create_bond(integration.id.unwrap(), &pko_bond).await?;

    assert_eq!(model.kind.unwrap(), "PL:ROR");
    assert_eq!(model.emission.unwrap(), "ROR0623");
    assert_eq!(model.name.unwrap(), "Obligacje roczne ROR");
    //assert_eq!(model.interest_date.unwrap(), NaiveDate::from_ymd(2022, 7, 1));

    Ok(())
  }
}
