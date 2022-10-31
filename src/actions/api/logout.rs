use rocket::{serde::json::Json, http::CookieJar};
use serde::Serialize;

use crate::stages::session::CurrentSession;

#[derive(Serialize, Debug)]
pub struct LogoutResponse {
  success: bool
}

#[get("/logout")]
pub async fn action(_current_session : CurrentSession, cookies: &CookieJar<'_>) -> Json<LogoutResponse> {
  let user_cookie = cookies.get_private("user_id");
  if let Some(cookie) = user_cookie {
    tracing::info!("clearing user id cookie");
    cookies.remove(cookie);
  }

  Json(
    LogoutResponse { success: true }
  )
}
