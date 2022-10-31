use entity::{QueryOrder, CategoryModel, Category, categories, EntityTrait};
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
pub struct AllCategoriesResponse {
  categories: Vec<CategoryModel>
}

#[tracing::instrument(level="info", skip(conn, _session))]
#[get("/")]
pub async fn action<'a>(conn: Connection<'a, Db>, _session : CurrentSession) -> ResponseResult<Json<AllCategoriesResponse>> {
  let db = conn.into_inner();
  let categories = Category::find()
    .order_by_asc(categories::Column::Name)
    .all(db)
    .await
    .map_err(catch_error)?;

  tracing::info!("Found: {} categories", categories.len());

  Ok(Json(AllCategoriesResponse { categories }))
}

#[cfg(test)]
mod test {
  use rocket::{http::Status, local::asynchronous::{Client, LocalResponse}};
  use crate::test;

  async fn response<'a>(client : &'a Client) -> LocalResponse<'a> {
    let response = client.get(format!("/api/categories"))
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
      "errors": vec!["You need to sign in to access: /api/categories"],
      "status": "forbidden"
    }).await;
  }
}
