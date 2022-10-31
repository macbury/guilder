use anyhow::Result;
use crypto::sha3::Sha3;
use crypto::digest::Digest;
use entity::ActiveValue;
use entity::sea_orm::DatabaseConnection;
use entity::user::{ActiveModel, Model};
use entity::ActiveModelTrait;

pub fn hash_password(password: &str, secret: &str) -> String {
  let mut hasher = Sha3::sha3_256();
  hasher.input_str(password);
  hasher.input_str(secret);
  hasher.result_str()
}

#[derive(Debug)]
pub struct NewUser {
  pub login: String,
  pub hashed_password: String
}

impl NewUser {
  pub fn new(login: &str, password: &str, secret: &str) -> Self {
    let hashed_password = hash_password(password, secret);
    Self {
      login: login.to_owned(),
      hashed_password
    }
  }

  pub async fn save(&self, db : &DatabaseConnection) -> Result<Model> {
    let model = ActiveModel { // use DeriveIntoActiveModel
      login: ActiveValue::Set(self.login.to_owned()),
      hashed_password: ActiveValue::Set(self.hashed_password.to_owned()),
      ..Default::default()
    };

    let entity = model.insert(db).await?;
    Ok(entity)
  }
}

#[cfg(test)]
mod test {
  use entity::{EntityTrait, User};
  use crate::test::db;
  use super::*;

  #[tokio::test]
  async fn initialize_new_user() -> Result<()> {
    let new_user = NewUser::new(
      "admin",
      "password",
      "secret"
    );

    assert_eq!(new_user.login, "admin");
    assert_eq!(new_user.hashed_password, "0c60bba8e2fef6ead455b23743bdf895bac6bb6dbed0c3c1b9718b0a9ccceb3d");

    let second_user = NewUser::new(
      "admin",
      "password",
      "other_secret"
    );
    assert_eq!(second_user.hashed_password, "9e1a6ef6e53733e58a3e83dc7bbc9427ab88923d5a1a1890eff953489c9370ef");

    let third_user = NewUser::new(
      "admin",
      "other_password",
      "other_secret"
    );

    assert_eq!(third_user.hashed_password, "265c70a056388f6220b32cea72a004f9369421f103a062484e2f59493abdd39a");

    Ok(())
  }

  #[tokio::test]
  async fn persist_user() -> Result<()> {
    let db = db().await?;

    let new_user = NewUser::new(
      "admin",
      "other_password",
      "other_secret"
    );

    let user = new_user.save(&db).await?;
    assert_eq!(user.login, "admin");
    assert_eq!(user.hashed_password, new_user.hashed_password);
    assert_eq!(user.id, 1);

    let users = User::find().all(&db).await?;
    assert_eq!(users.len(), 1);

    let user = users.get(0).unwrap();
    assert_eq!(user.login, "admin");
    assert_eq!(user.hashed_password, "265c70a056388f6220b32cea72a004f9369421f103a062484e2f59493abdd39a");
    assert_eq!(user.id, 1);

    Ok(())
  }
}
