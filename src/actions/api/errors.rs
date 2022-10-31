use rocket::Request;
use rocket::serde::{Serialize, json::Json};

#[derive(Serialize)]
pub struct ErrorsResponse<'a> {
  status: &'a str,
  errors: Vec<String>
}

#[catch(404)]
pub fn not_found<'a>(req: &Request) -> Json<ErrorsResponse<'a>> {
  let msg = format!("Not found: {}", req.uri());
  Json(
    ErrorsResponse {
      status: "not_found",
      errors: vec![msg]
    }
  )
}

#[catch(403)]
pub fn forbidden<'a>(req: &Request) -> Json<ErrorsResponse<'a>> {
  let msg = format!("You need to sign in to access: {}", req.uri());
  Json(
    ErrorsResponse {
      status: "forbidden",
      errors: vec![msg]
    }
  )
}

#[catch(422)]
pub fn unprocessable<'a>(_request: &Request) -> Json<ErrorsResponse<'a>> {
  Json(
    ErrorsResponse {
      status: "unprocessable",
      errors: vec!["Invalid request, check posted data".to_owned()]
    }
  )
}
