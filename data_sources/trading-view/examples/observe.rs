use anyhow::Result;
use trading_view::{Connection, Command, TradingViewEvent, Ticker};
use tokio::sync::mpsc::Sender;

async fn observe_symbol(connection : &mut Connection, symbol : &str) -> Result<()> {
  let ticker : Ticker = symbol.into();
  let chart_session = connection.generate_chart_id(&ticker).await;
  connection.send(Command::chart_create_session(&chart_session)).await?;

  connection.send(Command::set_time_zone(&chart_session)).await?;

  let quote_session = connection.generate_quote_session_id(&ticker).await;
  connection.send(Command::quote_create_session(&quote_session)).await?;

  let mut fields = vec![
    "country".to_owned(),
    "base-currency-logoid".to_owned(),
    "ch".to_owned(),
    "chp".to_owned(),
    "bid".to_owned(),
    "ask".to_owned(),
    "country_code".to_owned(),
    "currency-logoid".to_owned(),
    "currency_code".to_owned(),
    "description".to_owned(),
    "exchange".to_owned(),
    "is_tradable".to_owned(),
    "language".to_owned(),
    "local_description".to_owned(),
    "logoid".to_owned(),
    "original_name".to_owned(),
    "pro_name".to_owned(),
    "short_name".to_owned(),
    "type".to_owned(),
    "update_mode".to_owned(),
    "volume".to_owned(),
    "web_site_url".to_owned(),
    "business_description".to_owned(),
    "industry".to_owned(),
    "sector-i18n-en".to_owned()
  ];

  connection.send(
    Command::quote_add_symbols(
      &quote_session,
      symbol.try_into()?
    )?
  ).await?;

  connection.send(
    Command::resolve_symbol(
      &chart_session,
      symbol.try_into()?
    )?
  ).await?;

  connection.send(
    Command::quote_set_fields(
      &quote_session,
      &mut fields
    ),
  ).await?;

  connection.send(
    Command::quote_fast_symbols(
      &quote_session,
      symbol.try_into()?
    )?
  ).await?;

  connection.send(Command::create_series(&chart_session)).await?;

  // connection.send(
  //   Command::create_study(
  //     &chart_session,
  //     vec![
  //       "st1",
  //       "sessions_1",
  //       "sds_1",
  //       "Sessions@tv-basicstudies-156"
  //     ]
  //   )
  // ).await?;

  // connection.send(
  //   Command::create_study(
  //     &chart_session,
  //     vec![
  //       "st2",
  //       "st1",
  //       "sds_1",
  //       "Dividends@tv-basicstudies-156"
  //     ]
  //   )
  // ).await?;

  // connection.send(
  //   Command::create_study(
  //     &chart_session,
  //     vec![
  //       "st3",
  //       "st1",
  //       "sds_1",
  //       "Splits@tv-basicstudies-156"
  //     ]
  //   )
  // ).await?;

  // connection.send(
  //   Command::create_study(
  //     &chart_session,
  //     vec![
  //       "st4",
  //       "st1",
  //       "sds_1",
  //       "Earnings@tv-basicstudies-156"
  //     ]
  //   )
  // ).await?;

  // connection.send(
  //   Command::create_study(
  //     &chart_session,
  //     vec![
  //       "cs_IT5yew1atWZ1",
  //       "st5",
  //       "st1",
  //       "sds_1",
  //       "Volume@tv-basicstudies-156"
  //     ]
  //   )
  // ).await?;

  connection.start()?;

  Ok(())
}

async fn build_for(symbol: &str, sink: &Sender<TradingViewEvent>) -> Result<Connection> {
  let mut connection = Connection::new(sink).await?;

  connection.send(Command::set_auth_token()).await?;
  connection.send(Command::set_data_quality_high()).await?;

  observe_symbol(&mut connection, symbol).await?;

  Ok(connection)
}

#[tokio::main]
async fn main() -> Result<()> {
  tracing_subscriber::fmt::init();
  let (sink, mut stream) = tokio::sync::mpsc::channel::<TradingViewEvent>(32);

  // get subsessions -> "subsessions":[{"description":"Regular Trading Hours","id":"regular","private":false,"session":"0930-1600","session-display":"0930-1600"}
  // get timezone
  build_for( "BITSTAMP:BTCUSD", &sink).await?;
  build_for( "NASDAQ:TSLA", &sink).await?;
  build_for( "GPW:ALE", &sink).await?;
  //build_for( "PLNUSD", &sink).await?;

  loop {
    let event = stream.recv().await;

    match event {
      Some(TradingViewEvent::NewHistory(ticker, items)) => {
        tracing::info!("History size: {:?} {}", ticker, items.len());
      },
      Some(_) => tracing::info!("Event: {:?}", event),
      None => tracing::info!("Nothing...")
    }
  }
}
