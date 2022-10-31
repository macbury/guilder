use rocket::serde::json::Json;
use serde::Serialize;

use crate::stages::session::CurrentSession;

#[derive(Serialize, Debug)]
pub struct MeResponse {
  id: i32,
  login: String
}

#[get("/me")]
pub async fn action(current_session : CurrentSession) -> Json<MeResponse> {
  let user = current_session.user;

  Json(
    MeResponse { id: user.id, login: user.login }
  )
}

#[cfg(test)]
mod test {
  use rocket::{http::Status, local::asynchronous::{Client, LocalResponse}};
  use crate::test;

  async fn me<'a>(client : &'a Client) -> LocalResponse<'a> {
    let response = client.get("/api/me")
      .dispatch()
      .await;

    return response;
  }

  #[rocket::async_test]
  async fn reject_guest() {
    let (client, _db) = test::server().await;
    let response = me(&client).await;

    assert_eq!(response.status(), Status::Forbidden);
    assert_body!(response, {
      "errors": vec!["You need to sign in to access: /api/me"],
      "status": "forbidden"
    }).await;
  }

  #[rocket::async_test]
  async fn with_user() {
    let (client, db) = test::server().await;
    let (_user, _cookie) = test::sign_in(&client, &db).await;

    let response = client.get("/api/me")
      .dispatch()
      .await;

    assert_eq!(response.status(), Status::Ok);
    assert_body!(response, {
      "id": 1u8,
      "login": "admin"
    }).await;
  }
}
