use entity::{User, EntityTrait, ColumnTrait, QueryFilter, user};
use rocket::{serde::{Deserialize, Serialize, json::Json}, State};
use sea_orm_rocket::Connection;
use anyhow::anyhow;
use rocket::http::{Cookie, CookieJar};
use crate::{env::GuilderConfig, stages::Db, env::ResponseResult, services::new_user::hash_password};

#[derive(FromForm, Debug, Clone, Deserialize)]
pub struct SingInParams {
  login: String,
  password: String,
}

#[derive(Serialize, Debug)]
pub struct SignInResult {
  success: bool
}

//#[tracing::instrument(name = "sign_in", skip(config, conn, cookies))]
#[post("/sign_in", data = "<sign_in>")]
pub async fn action(sign_in: Json<SingInParams>, config: &State<GuilderConfig>, conn: Connection<'_, Db>, cookies: &CookieJar<'_>) -> ResponseResult<Json<SignInResult>> {
  let sign_in = sign_in.into_inner();
  let db = conn.into_inner();
  let user = User::find()
    .filter(user::Column::Login.eq(sign_in.login))
    .one(db)
    .await
    .map_err(|e| anyhow!(e))?;

  match user {
    Some(user) if user.hashed_password == hash_password(&sign_in.password, &config.salt) => {
      tracing::info!("Signing in: {}", user.id);
      let user_id : String = format!("{}", user.id);
      cookies.add_private(Cookie::new("user_id", user_id));
      Ok(Json(SignInResult { success: true }))
    },
    None | Some(_) => Ok(Json(SignInResult { success: false }))
  }
}

#[cfg(test)]
mod test {
  use rocket::{http::{ContentType, Status}, local::asynchronous::{Client, LocalResponse}};
  use crate::test;

  async fn sign_in<'a>(client : &'a Client, body : String) -> LocalResponse<'a> {
    let response = client.post("/api/sign_in")
      .header(ContentType::JSON)
      .body(body)
      .dispatch()
      .await;

    return response;
  }

  #[rocket::async_test]
  async fn invalid_params_and_login() {
    let (client, _db) = test::server().await;
    let response = sign_in(&client, json!({
      "login": "login",
      "password": "password"
    })).await;

    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.cookies().get_private("user_id"), None);

    assert_body!(response, {
      "success": false
    }).await;
  }

  #[rocket::async_test]
  async fn invalid_password() {
    let (client, db) = test::server().await;
    test::create_user("admin", "fake_password", &db).await;

    let response = sign_in(&client, json!({
      "login": "admin",
      "password": "valid_password"
    })).await;

    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.cookies().get_private("user_id"), None);

    assert_body!(response, {
      "success": false
    }).await;
  }

  #[rocket::async_test]
  async fn missing_params() {
    let (client, _db) = test::server().await;
    let response = sign_in(&client, json!({})).await;

    assert_body!(response, {
      "errors": vec!["Invalid request, check posted data"],
      "status": "unprocessable"
    }).await;
  }

  #[rocket::async_test]
  async fn valid_params() {
    let (client, db) = test::server().await;
    test::create_user("admin", "valid_password", &db).await;

    let response = sign_in(&client, json!({
      "login": "admin",
      "password": "valid_password"
    })).await;

    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.cookies().get_private("user_id").is_some(), true);

    assert_body!(response, {
      "success": true
    }).await;
  }
}
