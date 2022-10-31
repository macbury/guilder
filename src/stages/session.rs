use entity::sea_orm::DbErr;
use entity::{User, EntityTrait, user::Model};
use rocket::http::Status;
use rocket::{request::{Request, FromRequest, Outcome}};
use sea_orm_rocket::Database;

use super::Db;

#[derive(Debug)]
pub enum SessionError {
  InvalidToken,
  InternalError(DbErr),
}

#[derive(Clone, Debug)]
pub struct CurrentSession {
  pub user: Model
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for CurrentSession {
  type Error = SessionError;

  async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
    // https://api.rocket.rs/v0.5-rc/rocket/request/trait.FromRequest.html#request-local-state
    let cookies = req.cookies();
    let user_id : i32 = cookies.get_private("user_id")
      .and_then(|cookie| cookie.value().parse().ok())
      .unwrap_or(-1i32);

    tracing::trace!("Current user id: {}", user_id);
    let db = Db::fetch(&req.rocket())
      .expect("Missing database...");

    let user = User::find_by_id(user_id)
      .one(&db.conn)
      .await
      .map_err(|e| SessionError::InternalError(e));

    match user {
      Err(error) => Outcome::Failure((Status::Forbidden, error)),
      Ok(user) => {
        if let Some(user) = user {
          tracing::debug!("Signed in as user: {}", user.login);
          let session = CurrentSession { user };
          Outcome::Success(session)
        } else {
          tracing::debug!("Authentication is required");
          Outcome::Failure((Status::Forbidden, SessionError::InvalidToken))
        }
      }
    }
  }
}
