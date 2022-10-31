use rocket::{tokio::sync::broadcast::{channel, Sender}, fairing::AdHoc};
use trading_view::Ticker;

/**
 * Messages sent to user using SSE
 */
#[derive(Debug, Clone)]
pub enum ServerEvent {
  /**
   * Asset did change
   */
  AssetMetadataUpdated(Ticker),
  /**
   * Got new points from trading view
   */
  UpdatedPoints(Vec<Ticker>),
  /**
   * Integration status did change, first argument is integration id
   */
  UpdatedIntegration(i64),
}

pub type EventQueue = Sender<ServerEvent>;

/**
 * Configure message queue for SSE live events. Use managed EventQueue to push information back to user using /api/live endpoint
 */
pub fn stage() -> AdHoc {
  AdHoc::on_ignite("Events", |rocket| async {
    let (sender, _receiver) = channel::<ServerEvent>(1024);
    rocket.manage(sender)
  })
}
