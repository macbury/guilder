use entity::{AccountModel, AccountMetadataModel};
use validator::Validate;
use rocket::{serde::{Deserialize, Serialize, json::Json}};

#[derive(Validate, Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountParams {
  #[validate(length(min = 1, message = "minimal length is 1 character"))]
  pub name: String,
  pub description: String,
  #[validate(length(min = 1, message = "minimal length is 1 character"))]
  pub currency: String,
}

pub type JsonAccountParams = Json<AccountParams>;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountResource {
  #[serde(flatten)]
  account: AccountModel,
  #[serde(flatten)]
  metadata: Option<AccountMetadataModel>
}

impl From<(AccountModel, Option<AccountMetadataModel>)> for AccountResource {
  fn from((account, metadata): (AccountModel, Option<AccountMetadataModel>)) -> Self {
    Self { account, metadata }
  }
}

impl From<&(AccountModel, Option<AccountMetadataModel>)> for AccountResource {
  fn from((account, metadata): &(AccountModel, Option<AccountMetadataModel>)) -> Self {
    Self { account: account.clone(), metadata: metadata.clone() }
  }
}
