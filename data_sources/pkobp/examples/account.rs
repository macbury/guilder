extern crate dotenv;
use dotenv::dotenv;
use pkobp::PKOBPAccount;
use tokio;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
  dotenv().ok();
  tracing_subscriber::fmt::init();

  let username = std::env::var("PKOBP_USER").expect("Missing PKOBP_USER");
  let password = std::env::var("PKOBP_PASSWORD").expect("Missing PKOBP_PASSWORD");
  let mut account = PKOBPAccount::new("http://localhost:4444", &username, &password);
  account.sync().await?;

  tracing::info!("Number of bonds: {}", account.bonds.len());
  tracing::info!("Current cash: {}", account.cash);
  Ok(())
}
