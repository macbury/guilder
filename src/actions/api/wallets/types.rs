use entity::{WalletModel, WalletMetadataModel};
use validator::Validate;
use rocket::{serde::{Deserialize, Serialize, json::Json}};

#[derive(Validate, Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WalletParams {
  #[validate(length(min = 1, message = "minimal length is 1 character"))]
  pub name: String,
  #[validate(length(min = 1, message = "minimal length is 1 character"))]
  pub currency: String,
  pub description: String,
}

pub type JsonWalletParams = Json<WalletParams>;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WalletResource {
  #[serde(flatten)]
  account: WalletModel,
  #[serde(flatten)]
  metadata: Option<WalletMetadataModel>
}

impl From<(WalletModel, Option<WalletMetadataModel>)> for WalletResource {
  fn from((account, metadata): (WalletModel, Option<WalletMetadataModel>)) -> Self {
    Self { account, metadata }
  }
}

impl From<&(WalletModel, Option<WalletMetadataModel>)> for WalletResource {
  fn from((account, metadata): &(WalletModel, Option<WalletMetadataModel>)) -> Self {
    Self { account: account.clone(), metadata: metadata.clone() }
  }
}
