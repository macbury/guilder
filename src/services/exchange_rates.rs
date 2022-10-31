use entity::{sea_orm::DatabaseConnection, Wallet, EntityTrait, Account, Asset, Bond, ExchangeRate};
use trading_view::AssetsManager;
use anyhow::Result;
use itertools::{chain, Itertools};

#[derive(Clone)]
pub struct ExchangeRates {
  assets_manager: AssetsManager,
  db: DatabaseConnection
}

impl ExchangeRates {
  pub fn new(assets_manager : &AssetsManager, db: &DatabaseConnection) -> Self {
    Self {
      assets_manager: assets_manager.clone(),
      db: db.clone()
    }
  }

  pub async fn sync(&mut self) -> Result<()> {
    let target_currencies = self.target_currencies().await?;
    tracing::debug!("Currencies to: {:?}", target_currencies);
    let source_currencies = self.source_currencies().await?;
    tracing::debug!("Currencies from: {:?}", source_currencies);

    let currency_pairs : Vec<String> = source_currencies
      .iter()
      .cartesian_product(target_currencies)
      .map(|(from, to)| format!("{}{}", from, to))//TODO build currency? should it be like a ticker?
      .collect();

    tracing::debug!("Currencies pairs: {:?}", currency_pairs);

    for currency in currency_pairs {
      tracing::trace!("Observing currency pair: {}", currency);
      self.assets_manager.observe(currency.try_into()?).await?;
    }

    tracing::debug!("Waiting for currencies to be imported...");
    ExchangeRate::refresh(&self.db).await?;

    Ok(())
  }

  async fn target_currencies(&self) -> Result<Vec<String>> {
    let wallet_currencies : Vec<String> = Wallet::find()
      .all(&self.db)
      .await?
      .iter()
      .map(|m| m.currency.clone())
      .collect();
    let account_currencies : Vec<String> = Account::find()
      .all(&self.db)
      .await?
      .iter()
      .map(|m| m.currency.clone())
      .collect();

    let currencies : Vec<String> = chain(wallet_currencies, account_currencies)
      .unique()
      .collect();
    return Ok(currencies)
  }

  async fn source_currencies(&self) -> Result<Vec<String>> {
    let asset_currencies : Vec<String> = Asset::find()
      .all(&self.db)
      .await?
      .iter()
      .map(|m| m.currency.clone())
      .filter(|o| o.is_some())
      .map(|co| co.unwrap())
      .collect();
    let bond_currencies : Vec<String> = Bond::find()
      .all(&self.db)
      .await?
      .iter()
      .map(|m| m.currency.clone())
      .collect();

    let currencies : Vec<String> = chain(asset_currencies, bond_currencies)
      .unique()
      .collect();
    return Ok(currencies)
  }
}
