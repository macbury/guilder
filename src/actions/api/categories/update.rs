use entity::{categories, ActiveModelTrait, CategoryModel, Set, Category, EntityTrait};
use sea_orm_rocket::Connection;
use super::types::JsonCategoryParams;
use crate::{stages::{Db, session::CurrentSession}, env::{self, CrudResponse, CrudResult}};

#[tracing::instrument(level="info", skip(conn, _session))]
#[put("/<category_id>", data = "<params>")]
pub async fn action<'a>(_session : CurrentSession, category_id: i64, params: JsonCategoryParams, conn: Connection<'a, Db>) -> CrudResponse<CategoryModel> {
  let db = conn.into_inner();
  let params = params.into_inner();
  let category = Category::find_by_id(category_id)
    .one(db)
    .await
    .map_err(env::catch_error)?;

  if category.is_none() {
    return CrudResult::not_found();
  }

  if let Err((_, errors)) = env::validate(&params) {
    return CrudResult::fail(errors);
  }

  let mut model: categories::ActiveModel = category.unwrap().into();
  model.name = Set(params.name);

  tracing::debug!("Saving: {:?}", model);

  let resource = model
    .update(db)
    .await
    .map_err(env::catch_error)?;

  tracing::debug!("Created new category: {:?}", resource);

  CrudResult::success(resource)
}

#[cfg(test)]
mod test {
  use rocket::{http::Status, local::asynchronous::{Client, LocalResponse}};
  use crate::test;

  async fn response<'a>(client : &'a Client) -> LocalResponse<'a> {
    let response = client.put(format!("/api/categories/1"))
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
