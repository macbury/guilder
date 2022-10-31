use entity::{comments, ActiveModelTrait, CommentModel, ActiveValue::NotSet, Set};
use rocket::{serde::{Deserialize, Serialize, json::{Json}}};
use sea_orm_rocket::Connection;
use validator::Validate;
use crate::{stages::{Db, session::CurrentSession}, env::{self, CrudResponse, CrudResult}, utils};

#[derive(Validate, Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CommentParams {
  #[validate(length(min = 1, message = "minimal length is 1 character"))]
  body: String,
  model_id: String,
  model_type: String,
}

#[tracing::instrument(level="info", skip(conn, session))]
#[post("/", data = "<comment>")]
pub async fn action<'a>(session : CurrentSession, comment: Json<CommentParams>, conn: Connection<'a, Db>) -> CrudResponse<CommentModel> {
  let comment = comment.into_inner();
  if let Err((_, errors)) = env::validate(&comment) {
    return CrudResult::fail(errors);
  }

  let form : comments::ActiveModel = comments::ActiveModel {
    id: NotSet,
    date: Set(utils::now()),
    body: Set(comment.body),
    model_id: Set(comment.model_id),
    model_type: Set(comment.model_type),
    user_id: Set(session.user.id),
    ..Default::default()
  };

  tracing::debug!("Saving: {:?}", form);

  let model = form
    .insert(conn.into_inner())
    .await
    .map_err(env::catch_error)?;

  tracing::debug!("Created new comment: {:?}", model);

  CrudResult::success(model)
}


#[cfg(test)]
mod test {
  use rocket::{http::Status, local::asynchronous::{Client, LocalResponse}};
  use crate::test;

  async fn response<'a>(client : &'a Client) -> LocalResponse<'a> {
    let response = client.post(format!("/api/comments/"))
      .dispatch()
      .await;

    return response;
  }

  #[rocket::async_test]
  async fn reject_guest() {
    let (client, _db) = test::server().await;
    let response = response(&client).await;

    assert_eq!(response.status(), Status::Forbidden);
    assert_body!(response, {
      "errors": vec!["You need to sign in to access: /api/comments/"],
      "status": "forbidden"
    }).await;
  }
}
