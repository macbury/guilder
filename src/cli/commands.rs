use crate::services::NewUser;
use anyhow::Result;
use entity::sea_orm::Database;
use rocket::figment::Figment;

pub async fn create_user(username: &str, password: &str, config : &Figment) -> Result<bool> {
  let url : String = config.extract_inner("databases.guilder.url")?;
  let conn = Database::connect(url).await?;
  let secret : String = config.extract_inner("secret_key")?;
  let new_user = NewUser::new(&username, &password, &secret);
  let user = new_user.save(&conn).await?;
  println!("User created: {}", user.login);

  Ok(true)
}
