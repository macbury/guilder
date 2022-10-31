use entity::{categories, ActiveModelTrait, CategoryModel, ActiveValue::NotSet, Set};
use sea_orm_rocket::Connection;
use super::types::JsonCategoryParams;
use crate::{stages::{Db, session::CurrentSession}, env::{self, CrudResponse, CrudResult}};

#[tracing::instrument(level="info", skip(conn, _session))]
#[post("/", data = "<category>")]
pub async fn action<'a>(_session : CurrentSession, category: JsonCategoryParams, conn: Connection<'a, Db>) -> CrudResponse<CategoryModel> {
  let category = category.into_inner();
  if let Err((_, errors)) = env::validate(&category) {
    return CrudResult::fail(errors);
  }

  let model = categories::ActiveModel {
    id: NotSet,
    name: Set(category.name),
    ..Default::default()
  };

  tracing::debug!("Saving: {:?}", model);

  let resource = model
    .insert(conn.into_inner())
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
    let response = client.post(format!("/api/categories"))
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
