use rocket::serde::json::Json;
use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct HealthResponse {
  health: bool,
  message: String
}

#[get("/health")]
pub async fn action() -> Json<HealthResponse> {
  Json(
    HealthResponse { health: true, message: "Hello :)".to_string() }
  )
}
