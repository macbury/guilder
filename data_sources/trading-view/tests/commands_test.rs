use trading_view::Command;
use anyhow::Result;

#[test]
fn it_serializes_command() -> Result<()> {
  let cmd = Command::set_auth_token();

  let serialized = serde_json::to_string(&cmd)?;
  assert_eq!("{\"m\":\"set_auth_token\",\"p\":[\"unauthorized_user_token\"]}", serialized);
  Ok(())
}

#[test]
fn it_generates_msg() -> Result<()> {
  let cmd = Command::set_auth_token();
  let msg : String = cmd.try_into()?;

  assert_eq!("~m~54~m~{\"m\":\"set_auth_token\",\"p\":[\"unauthorized_user_token\"]}", msg);
  Ok(())
}
