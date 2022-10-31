use crate::stages::Db;
use crate::types::{AssetDetails};
use crate::stages::events::{EventQueue, ServerEvent};
use crate::stages::session::CurrentSession;
use entity::sea_orm::DatabaseConnection;
use entity::{AssetPerformance, EntityTrait, Expr, QueryFilter, Asset, assets, IntegrationModel, Integration};
use rocket::tokio::select;
use rocket::{State, Shutdown};
use rocket::response::stream::{EventStream, Event};
use rocket::tokio::sync::broadcast::error::RecvError;
use sea_orm_rocket::Connection;
use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub enum LiveEvent {
  Assets(Vec<AssetDetails>),
  Integration(IntegrationModel)
}

async fn process_event(event : ServerEvent, conn: &DatabaseConnection) -> anyhow::Result<Option<Event>> {
  match event {
    ServerEvent::AssetMetadataUpdated(ticker) => {
      let asset = Asset::find_by_id(ticker.into())
        .find_also_related(AssetPerformance)
        .one(conn)
        .await?;

      if let Some(asset) = asset {
        let asset_details : AssetDetails = asset.into();
        let response = LiveEvent::Assets(vec![asset_details]);
        return Ok(Some(Event::json(&response)))
      }
    },
    ServerEvent::UpdatedPoints(tickers) => {
      let tickers : Vec<String> = tickers.iter().map(|ticker| ticker.into()).collect();
      let assets : Vec<AssetDetails> = Asset::find()
        .filter(Expr::col(assets::Column::Id).is_in(tickers))
        .find_also_related(AssetPerformance)
        .all(conn)
        .await?
        .iter()
        .map(|asset| asset.into())
        .collect();

      return Ok(Some(Event::json(&LiveEvent::Assets(assets))));
    },
    ServerEvent::UpdatedIntegration(id) => {
      let integration = Integration::find_by_id(id).one(conn).await?;

      if let Some(integration) = integration {
        return Ok(Some(Event::json(&LiveEvent::Integration(integration))));
      } else {
        tracing::error!("Integration with id {} was not found, skipping push", id);
      }
    }
  }

  return Ok(None)
}

#[get("/live")]
pub async fn action<'a>(conn: Connection<'a, Db>, queue: &State<EventQueue>, _cs : CurrentSession, mut end: Shutdown) -> EventStream![] {
  let mut rx = queue.subscribe();
  let conn = conn.into_inner().clone();

  EventStream! {
    loop {
      let event = select! {
        msg = rx.recv() => match msg {
          Ok(msg) => msg,
          Err(RecvError::Closed) => break,
          Err(RecvError::Lagged(_)) => continue,
        },
        _ = &mut end => break,
      };

      let event = process_event(event, &conn).await;

      match event {
        Ok(event) if event.is_some() => {
          yield event.unwrap()
        },
        Ok(_) => {},
        Err(err) => {
          tracing::error!("Could not send event over live SSE: {:?}", err);
          break;
        }
      }
    }
  }
}

#[cfg(test)]
mod test {
  use rocket::{http::Status, local::asynchronous::{Client, LocalResponse}};
  use crate::test;

  async fn live<'a>(client : &'a Client) -> LocalResponse<'a> {
    let response = client.get("/api/live")
      .dispatch()
      .await;

    return response;
  }

  #[rocket::async_test]
  async fn reject_guest() {
    let (client, _db) = test::server().await;
    let response = live(&client).await;

    assert_eq!(response.status(), Status::Forbidden);
    assert_body!(response, {
      "errors": vec!["You need to sign in to access: /api/live"],
      "status": "forbidden"
    }).await;
  }
}
