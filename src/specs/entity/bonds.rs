#[cfg(test)]
mod test {
  use entity::{Bond, NotSet, Set, bonds::{self, BondStatus}, ActiveModelBehavior, ActiveModelTrait, integrations::{self, Kind}, Unchanged};
  use anyhow::Result;
  use crate::{test::db, utils::{today, now}};

  #[tokio::test]
  async fn test_initialize_new_record() -> Result<()> {
    let db = db().await?;
    let bond = Bond::find_or_initialize_by_uid("abcd", 123, &db).await?;

    assert_eq!(bond.id, NotSet);
    assert_eq!(bond.uid, Set("abcd".to_owned()));
    assert_eq!(bond.integration_id, Set(Some(123)));
    Ok(())
  }

  #[tokio::test]
  async fn test_use_existing_bond() -> Result<()> {
    let db = db().await?;
    let mut integration = integrations::ActiveModel::new();
    integration.name = Set("Testing".to_owned());
    integration.login = Set("Login".to_owned());
    integration.kind = Set(Kind::PKOBP);
    let integration = integration.save(&db).await?;
    let integration_id : i64 = integration.id.unwrap();

    let mut existing_bond = bonds::ActiveModel::new();
    existing_bond.uid = Set("abcd".to_owned());
    existing_bond.name = Set("name".to_owned());
    existing_bond.integration_id = Set(Some(integration_id));
    existing_bond.emission = Set("COI1234".to_owned());
    existing_bond.kind = Set("COI".to_owned());
    existing_bond.start_date = Set(today());
    existing_bond.currency = Set("PLN".to_owned());
    existing_bond.status = Set(BondStatus::Active);
    existing_bond.interest_date = Set(today());
    existing_bond.end_date = Set(today());
    existing_bond.updated_at = Set(now());

    let existing_bond = existing_bond.save(&db).await?;
    let bond = Bond::find_or_initialize_by_uid("abcd", integration_id, &db).await?;

    assert_eq!(bond.id, existing_bond.id);
    assert_eq!(bond.uid, Unchanged("abcd".to_owned()));
    assert_eq!(bond.integration_id, Unchanged(Some(integration_id)));
    Ok(())
  }
}
