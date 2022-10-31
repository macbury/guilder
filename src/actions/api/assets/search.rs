use crate::{stages::session::CurrentSession, env::ResponseResult};
use rocket::{serde::json::Json};
use serde::Serialize;
use trading_view::{AssetsManager, SearchResult, Ticker, SymbolType};

#[derive(Serialize, Debug)]
pub struct SearchResponse {
  results: Vec<SearchResultItem>
}

#[derive(Serialize, Debug)]
pub struct SearchResultItem {
  ticker: String,
  description: String,
  kind: SymbolType,
  logo: Option<String>,
  country: Option<String>
}

impl From<SearchResult> for SearchResultItem {
  fn from(sr: SearchResult) -> Self {
    let ticker : Ticker = sr.clone().into();
    Self {
      kind: sr.kind,
      ticker: ticker.try_into().unwrap_or_default(),
      description: voca_rs::strip::strip_tags(&sr.description),
      logo: sr.logo_id.map(|logo_id| format!("https://s3-symbol-logo.tradingview.com/{}.svg", logo_id)),
      country: sr.country.map(|country| format!("https://s3-symbol-logo.tradingview.com/country/{}.svg", country))
    }
  }
}

#[get("/search?<query>")]
pub async fn action<'a>(_session : CurrentSession, query: Option<&'a str>) -> ResponseResult<Json<SearchResponse>> {
  let query = query.unwrap_or_default();
  if !query.is_empty() {
    let results = AssetsManager::search(query)
      .await?
      .iter()
      .map(|result| result.to_owned().into())
      .collect();

    Ok(Json(SearchResponse { results }))
  } else {
    Ok(Json(SearchResponse { results: vec![] }))
  }
}

#[cfg(test)]
mod test {
  use rocket::{http::{ContentType, Status}, local::asynchronous::{Client, LocalResponse}};
  use crate::test;

  async fn search<'a>(client : &'a Client, query : String) -> LocalResponse<'a> {
    let response = client.get(format!("/api/assets/search?query={}", query))
      .header(ContentType::JSON)
      .dispatch()
      .await;

    return response;
  }

  #[rocket::async_test]
  async fn reject_guest() {
    let (client, _db) = test::server().await;
    let response = search(&client, "GPW:GPW".to_owned()).await;

    assert_eq!(response.status(), Status::Forbidden);
    assert_body!(response, {
      "errors": vec!["You need to sign in to access: /api/assets/search?query=GPW:GPW"],
      "status": "forbidden"
    }).await;
  }
}
