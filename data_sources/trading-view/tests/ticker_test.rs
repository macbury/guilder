use trading_view::Ticker;
use anyhow::Result;

#[test]
fn it_transform_string_into_ticker() -> Result<()> {
  let text: String = "BITSTAMP:BTCUSD".to_owned();
  let ticker: Ticker = text.try_into()?;

  assert_eq!(ticker.currency, false);
  assert_eq!(ticker.exchange, "BITSTAMP");
  assert_eq!(ticker.symbol, "BTCUSD");
  Ok(())
}

#[test]
fn it_transform_string_into_currency_ticker() -> Result<()> {
  let text: String = "PLNUSD".to_owned();
  let ticker: Ticker = text.try_into()?;

  assert_eq!(ticker.currency, true);
  assert_eq!(ticker.exchange, "");
  assert_eq!(ticker.symbol, "PLNUSD");
  Ok(())
}

#[test]
fn it_ticker_into_string() -> Result<()> {
  let ticker: Ticker = Ticker::new(Some("NASDAQ"), "INTL");
  let result : String = ticker.try_into()?;

  assert_eq!(result, "NASDAQ:INTL");
  Ok(())
}

#[test]
fn it_ticker_currency_into_string() -> Result<()> {
  let ticker: Ticker = Ticker::new(None, "USDPLN");
  let result : String = ticker.try_into()?;

  assert_eq!(result, "USDPLN");
  Ok(())
}
