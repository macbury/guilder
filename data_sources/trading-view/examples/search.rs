use anyhow::{Result, Ok};
use trading_view::AssetsManager;


#[tokio::main]
async fn main() -> Result<()> {
  tracing_subscriber::fmt::init();
  println!("appl {:#?}", AssetsManager::search("appl").await?);
  println!("tsla {:#?}", AssetsManager::search("tsla").await?);
  println!("gold {:#?}", AssetsManager::search("gold").await?);
  println!("crude {:#?}", AssetsManager::search("crude").await?);
  println!("btc {:#?}", AssetsManager::search("btc").await?);
  println!("ishare {:#?}", AssetsManager::search("ishare").await?);
  println!("PLNUSD {:#?}", AssetsManager::search("PLNUSD").await?);
  Ok(())
}
