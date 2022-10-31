use std::collections::HashMap;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64};

use anyhow::{Result, Context, anyhow};
use once_cell::sync::Lazy;
use futures_util::{SinkExt, StreamExt};
use tokio::net::TcpStream;
use tokio::sync::{Mutex, RwLock};
use tokio::time::{sleep, timeout};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tokio_tungstenite::{connect_async, MaybeTlsStream, WebSocketStream};

use tokio_tungstenite::tungstenite::handshake::client::Request;
use tungstenite::Message;
use tungstenite::handshake::client::generate_key;
use regex::Regex;
use tokio::sync::mpsc::Sender;
use rand::{distributions::Alphanumeric, Rng};

use crate::Ticker;
use crate::commands::{Response, Command, TradingViewEvent};

const ENDPOINT : &str = "wss://data.tradingview.com/socket.io/websocket";

static PING : Lazy<Regex> = Lazy::new(|| {
  Regex::new(r"~m~\d+~m~~h~\d+").expect("Ping Regexp invalid...")
});

static DELIMITER : Lazy<Regex> = Lazy::new(|| {
  Regex::new(r"~m~\d+~m~").expect("DELIMITER Regexp invalid...")
});

fn build_req() -> Result<Request> {
  let req = Request::builder()
      .method("GET")
      .header("Host", " data.tradingview.com")
      .header("Origin", "https://www.tradingview.com")
      .header("Connection", "Upgrade")
      .header("Upgrade", "websocket")
      .header("Sec-WebSocket-Version", "13")
      .header("Sec-WebSocket-Key", generate_key())
      .uri(ENDPOINT)
      .body(())
    .with_context(|| "Could not initialize request for connecting to tradingview")?;

  Ok(req)
}

#[derive(Clone)]
pub struct Connection {
  socket: Arc<Mutex<WebSocketStream<MaybeTlsStream<TcpStream>>>>,
  sink: Sender<TradingViewEvent>,
  last_msg: Arc<AtomicU64>,
  chart_sessions: Arc<RwLock<HashMap<String, Ticker>>>,
  quote_sessions: Arc<RwLock<HashMap<String, Ticker>>>,
}

fn now() -> u64 {
  SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()
}

fn generate_uid(prefix : &str) -> String {
  let hash : String = rand::thread_rng()
    .sample_iter(&Alphanumeric)
    .take(12)
    .map(char::from)
    .collect();

  format!("{}_{}", prefix, hash)
}

impl Connection {
  pub async fn new(sink: &Sender<TradingViewEvent>) -> Result<Self> {
    let req = build_req()?;

    // initialize unbound channel for events
    tracing::debug!("Connecting {:?}", req);
    let req = connect_async(req);
    let (socket, _) = timeout(Duration::from_secs(10), req).await
      .with_context(|| "Timeout connection")?
      .with_context(|| "Could not open connection")?;

    Ok(
      Self {
        last_msg: Arc::new(AtomicU64::new(now())),
        socket: Arc::new(Mutex::new(socket)),
        sink: sink.clone(),
        chart_sessions: Arc::new(RwLock::new(HashMap::new())),
        quote_sessions: Arc::new(RwLock::new(HashMap::new())),
      }
    )
  }

  pub async fn find_qid(&mut self, searched_ticker: &Ticker) -> Option<String> {
    let quote_sessions = self.quote_sessions.read().await;
    for (qid, ticker) in quote_sessions.iter() {
      if ticker == searched_ticker {
        return Some(qid.to_owned())
      }
    }

    return None
  }

  pub async fn generate_chart_id(&mut self, ticker : &Ticker) -> String {
    let mut chart_sessions = self.chart_sessions.write().await;
    let chart_session_id = generate_uid("cs");
    chart_sessions.insert(chart_session_id.clone(), ticker.clone());
    return chart_session_id;
  }

  pub async fn generate_quote_session_id(&mut self, ticker : &Ticker) -> String {
    let mut quote_sessions = self.quote_sessions.write().await;
    let quote_session_id = generate_uid("qs");
    quote_sessions.insert(quote_session_id.clone(), ticker.clone());
    return quote_session_id;
  }

  pub async fn is_dead(&self) -> bool {
    let last_msg = self.last_msg.load(std::sync::atomic::Ordering::Relaxed);
    let dead_time = 30;
    let elapsed = now() - last_msg;
    tracing::trace!("Elapsed from last update: {:?}", elapsed);
    return elapsed >= dead_time;
  }

  pub fn start(&mut self) -> Result<bool> {
    let mut this = self.clone();
    tokio::spawn(async move {
      tracing::trace!("Started tokio task in thread");

      loop {
        if let Err(err) = this.read().await {
          tracing::error!("Connection failed: {}, waiting 1 seconds before killing it", err);
          sleep(Duration::from_secs(1)).await;
          break;
        }
      }

      tracing::info!("Closing connection");
    });

    Ok(true)
  }

  pub async fn send(&mut self, cmd : Command) -> Result<bool> {
    tracing::trace!("Locked connection...");
    let mut socket = self.socket.lock().await;
    let msg = Message::Text(cmd.try_into()?);
    tracing::debug!("Sending: {}", msg);
    socket.send(msg).await
      .with_context(|| "could not send command to trading view")?;
    tracing::trace!("Send success");
    Ok(true)
  }

  async fn next(&mut self) -> Option<Result<Message, tungstenite::Error>> {
    let socket = self.socket.try_lock();

    if let Ok(mut socket) = socket {
      tokio::select! {
        msg = socket.next() => {
          if msg.is_some() {
            self.last_msg.store(now(), std::sync::atomic::Ordering::Relaxed);
          }

          return msg;
        },
        _ = sleep(Duration::from_millis(200)) => {
          return None
        }, // if you get x number of timeouts then mark as failed connection
      }
    } else {
      // wait one millisecond to prevent loop that reads to block forever socket
      sleep(Duration::from_millis(33)).await;
      None
    }
  }

  async fn ping(&mut self, ping_response : Message) -> Result<()> {
    tracing::debug!("Ping");
    let mut socket = self.socket.lock().await;
    socket.send(ping_response).await
      .with_context(|| "Could not send ping back!")?;

    Ok(())
  }

  async fn read(&mut self) -> Result<()> {
    //tracing::trace!("Waiting for next message...");
    let msg = self.next().await;

    if let Some(msg) = msg {
      match msg {
        Err(error) => {
          tracing::error!("Problem while reading next msg: {:?}", error);
          return Err(anyhow!(error))
        },

        Ok(msg) => {
          let ping_response = msg.clone();
          let data = msg.into_text()
            .with_context(|| "Could not get websocket message body")?;

          //tracing::trace!("Received data: {:?}", data);

          if PING.is_match(&data) {
            self.ping(ping_response).await?;
          }

          for msg in DELIMITER.split(&data) {
            if msg.len() < 10 {
              continue;
            }

            // tracing::trace!("Got part: {}", msg);
            if let Err(problem) = self.process(&msg).await {
              tracing::trace!("Could not process msg: {:?}", problem)
            }
          }
        }
      }
    }

    Ok(())
  }

  async fn process(&self, msg: &str) -> Result<bool> {
    let response: Response = serde_json::from_str(msg)?;

    match response.name.as_str() {
      "qsd" => {
        tracing::debug!("DATA: {}", msg);
        response.to_ticker_data(&self.sink).await?;
      },
      "timescale_update" => {
        let chart_sessions = self.chart_sessions.read().await;
        response.to_timescale_update(&self.sink, &chart_sessions).await?;
      },
      "symbol_resolved" => {
        tracing::debug!("DATA: {}", msg);
        response.to_symbol_resolved(&self.sink).await?
      },
      _ => {
        return Err(anyhow!("Could not build trading view for msg: {:?}", response.name))
      }
    }


    Ok(true)
  }
}
