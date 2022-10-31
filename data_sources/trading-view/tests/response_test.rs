use trading_view::{commands::Response, TradingViewEvent, Ticker};

use anyhow::{Result, Context};
use tokio::sync::mpsc::{Sender, Receiver};
use std::{fs, collections::HashMap};

fn parse_response(path: &str) -> Result<(Response, Sender<TradingViewEvent>, Receiver<TradingViewEvent>)> {
  let contents = fs::read_to_string(path)
    .with_context(|| "Could not load fixture")?; // "./fixtures/symbol_resolved.json"
  let response: Response = serde_json::from_str(&contents)
    .with_context(|| "Could not deserialize into Response")?;

  let (sink, stream) = tokio::sync::mpsc::channel::<TradingViewEvent>(100);

  Ok((response, sink, stream))
}

fn read_event(stream : &mut Receiver<TradingViewEvent>) -> TradingViewEvent {
  tokio_test::block_on(stream.recv()).unwrap()
}

#[test]
fn it_generates_event_with_description() -> Result<()> {
  let (response, sink, mut stream) = parse_response("./tests/fixtures/symbol_resolved.json")?;
  tokio_test::block_on(response.to_symbol_resolved(&sink))?;

  let ticker: Ticker = "NASDAQ:TSLA".try_into()?;

  assert_eq!(read_event(&mut stream), TradingViewEvent::Isin(ticker.clone(), "US88160R1014".to_string()));

  Ok(())
}

#[test]
fn it_qsd_basic_info() -> Result<()> {
  let (response, sink, mut stream) = parse_response("./tests/fixtures/qsd/ch.json")?;
  let ticker: Ticker = "NASDAQ:TSLA".try_into()?;

  tokio_test::block_on(response.to_ticker_data(&sink))?;

  assert_eq!(read_event(&mut stream), TradingViewEvent::Volume(ticker.clone(), 9088395.0));
  assert_eq!(read_event(&mut stream), TradingViewEvent::CurrencyCode(ticker.clone(), "USD".to_owned()));
  assert_eq!(read_event(&mut stream), TradingViewEvent::Logo(ticker.clone(), "https://s3-symbol-logo.tradingview.com/tesla--big.svg".to_owned()));
  assert_eq!(read_event(&mut stream), TradingViewEvent::Title(ticker.clone(), "Tesla, Inc.".to_owned()));
  Ok(())
}


#[test]
fn it_qsd_description() -> Result<()> {
  let (response, sink, mut stream) = parse_response("./tests/fixtures/qsd/meta.json")?;
  let ticker: Ticker = "NASDAQ:TSLA".try_into()?;

  tokio_test::block_on(response.to_ticker_data(&sink))?;

  assert_eq!(read_event(&mut stream), TradingViewEvent::WebsiteUrl(ticker.clone(), "http://www.tesla.com".to_owned()));
  assert_eq!(read_event(&mut stream), TradingViewEvent::Description(ticker.clone(), "tesla about it".to_owned()));
  Ok(())
}

#[test]
fn it_parse_timescale_update() -> Result<()> {
  tracing_subscriber::fmt::init();
  let mut sessions : HashMap<String, Ticker> = HashMap::new();
  sessions.insert("cs_YE4Hp8GpSouZ".to_string(), Ticker::new(Some("NAS"), "test"));

  let (response, sink, mut stream) = parse_response("./tests/fixtures/timescale_update.json")?;

  tokio_test::block_on(response.to_timescale_update(&sink, &sessions))?;

  let evt = read_event(&mut stream);

  if let TradingViewEvent::NewHistory(ticker, history) = evt {
    assert_eq!("NAS:TEST".to_string(), ticker.to_s());
    assert_eq!(2957, history.len());

    let entry_first = history[0];

    assert_eq!(entry_first.volume, 93916400.0);
    assert_eq!(entry_first.time.to_string(), "2010-06-29 13:30:00 UTC");
    assert_eq!(entry_first.close_price, 4.778);
    assert_eq!(entry_first.opening_price, 3.8);
    assert_eq!(entry_first.highest_price, 5.0);
    assert_eq!(entry_first.lowest_price, 3.507998);

    let last_first = history.last().unwrap();

    assert_eq!(last_first.volume, 20677182.0);
    assert_eq!(last_first.time.to_string(), "2022-03-25 13:30:00 UTC");
    assert_eq!(last_first.close_price, 1010.64);
    assert_eq!(last_first.opening_price, 1008.0);
    assert_eq!(last_first.highest_price, 1021.7999);
    assert_eq!(last_first.lowest_price, 997.3201);
  } else {
    assert!(false, "Evt is not TradingViewEvent::NewHistory");
  }

  Ok(())
}
