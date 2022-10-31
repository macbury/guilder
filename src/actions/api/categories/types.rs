use validator::Validate;
use rocket::{serde::{Deserialize, Serialize, json::Json}};

#[derive(Validate, Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CategoryParams {
  #[validate(length(min = 1, message = "minimal length is 1 character"))]
  pub name: String,
}

pub type JsonCategoryParams = Json<CategoryParams>;
