use trading_view::{AssetsManager, SymbolType, Ticker};
use anyhow::Result;

#[tokio::test]
async fn it_searches_apple() -> Result<()> {
  let results = AssetsManager::search("AAPL").await?;
  let best = results[0].to_owned();
  assert_eq!(best.symbol, "<em>AAPL</em>");
  assert_eq!(best.description, "Apple Inc.");
  assert_eq!(best.exchange, "NASDAQ");
  assert_eq!(best.kind, SymbolType::Stock);
  assert_eq!(best.country, Some("US".to_owned()));
  assert_eq!(best.logo_id, Some("apple".to_owned()));
  assert_eq!(best.base_logo_id, None);

  let ticker : Ticker = best.into();
  assert_eq!(ticker.symbol, "AAPL");
  assert_eq!(ticker.exchange, "NASDAQ");
  assert_eq!(ticker.to_s(), "NASDAQ:AAPL");

  Ok(())
}
