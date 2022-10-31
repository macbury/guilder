use entity::integrations::Kind;
use validator::Validate;
use rocket::{serde::{Deserialize, Serialize, json::Json}};

#[derive(Validate, Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct IntegrationsParams {
  #[validate(length(min = 1, message = "minimal length is 1 character"))]
  pub name: String,
  #[validate(required)]
  pub login: Option<String>,
  pub password: Option<String>,
  #[validate(required)]
  pub kind: Option<Kind>,
}

pub type JsonIntegrationsParams = Json<IntegrationsParams>;
