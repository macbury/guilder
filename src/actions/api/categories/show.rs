use entity::{CategoryModel, Category, EntityTrait};
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
pub struct ShowCategoryResponse {
  category: CategoryModel
}

#[tracing::instrument(level="info", skip(conn, _session))]
#[get("/<category_id>")]
pub async fn action<'a>(conn: Connection<'a, Db>, _session : CurrentSession, category_id: i64) -> ResponseResult<Option<Json<ShowCategoryResponse>>> {
  let db = conn.into_inner();
  let category = Category::find_by_id(category_id)
    .one(db)
    .await
    .map_err(catch_error)?
    .map(|category| Json(ShowCategoryResponse { category }));

  tracing::info!("Found category: {:?}", category);
  Ok(category)
}

#[cfg(test)]
mod test {
  use rocket::{http::Status, local::asynchronous::{Client, LocalResponse}};
  use crate::test;

  async fn response<'a>(client : &'a Client) -> LocalResponse<'a> {
    let response = client.get(format!("/api/categories/1"))
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
      "errors": vec!["You need to sign in to access: /api/categories/1"],
      "status": "forbidden"
    }).await;
  }
}
