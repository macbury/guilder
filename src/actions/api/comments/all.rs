use entity::{Comment, QueryOrder, comments, CommentModel};
use rocket::serde::json::Json;
use sea_orm_rocket::Connection;
use serde::Serialize;

use crate::{
  stages::{
    session::CurrentSession,
    Db
  },
  env::{ResponseResult, catch_error},
};

#[derive(Serialize, Debug)]
pub struct AllCommentsResponse {
  comments: Vec<CommentModel>
}

#[tracing::instrument(level="info", skip(conn, _session))]
#[get("/<model_type>/<model_id>")]
pub async fn action<'a>(conn: Connection<'a, Db>, _session : CurrentSession, model_type: &'a str, model_id: &'a str) -> ResponseResult<Json<AllCommentsResponse>> {
  let db = conn.into_inner();
  let comments = Comment::find_for_model(model_id, model_type)
    .order_by_desc(comments::Column::Date)
    .all(db)
    .await
    .map_err(catch_error)?;

  tracing::info!("Found: {} comments", comments.len());

  Ok(Json(AllCommentsResponse { comments }))
}

#[cfg(test)]
mod test {
  use rocket::{http::Status, local::asynchronous::{Client, LocalResponse}};
  use crate::test;

  async fn response<'a>(client : &'a Client, id: i64) -> LocalResponse<'a> {
    let response = client.get(format!("/api/comments/test/{}", id))
      .dispatch()
      .await;

    return response;
  }

  #[rocket::async_test]
  async fn reject_guest() {
    let (client, _db) = test::server().await;
    let response = response(&client, 1).await;

    assert_eq!(response.status(), Status::Forbidden);
    assert_body!(response, {
      "errors": vec!["You need to sign in to access: /api/comments/test/1"],
      "status": "forbidden"
    }).await;
  }
}
