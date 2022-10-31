use anyhow::Result;
use trading_view::{AssetsManager, TradingViewEvent};

#[tokio::main]
async fn main() -> Result<()> {
  tracing_subscriber::fmt::init();
  let (mut am, mut stream) = AssetsManager::new();

  am.observe("NASDAQ:TSLA".try_into()?).await?;
  am.observe("PLNUSD".try_into()?).await?;
  am.observe("EURPLN".try_into()?).await?;
  am.observe("USDEUR".try_into()?).await?;
  am.observe("BITSTAMP:BTCUSD".try_into()?).await?;
  am.observe("GPW:GPW".try_into()?).await?;
  am.observe("TVC:GOLD".try_into()?).await?;

  loop {
    let event = stream.recv().await;

    match event {
      Some(TradingViewEvent::NewHistory(_chart_session_id, items)) => {
        tracing::info!("(Thread: 2) History size: {}", items.len());
      },
      Some(_) => tracing::info!("(Thread: 2) Event: {:?}", event),
      None => {}
    }
  }
}
