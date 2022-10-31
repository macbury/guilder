use chrono::NaiveDate;
use anyhow::Result;
use sea_orm::{DatabaseConnection, DatabaseBackend, Statement, sea_query::{OnConflict, Query, PostgresQueryBuilder, Expr, InsertStatement}, ConnectionTrait};
use crate::points::{Kind, Entity, Column};

const FLUSH_EVERY : i32 = 3000;

/**
 * Generic points importer, it flushes all points every FLUSH_EVERY size. Just be sure to run commit at the end.
 */
pub struct ImportDataPoints<'a> {
  db: &'a DatabaseConnection,
  resource_type: &'a str,
  points_left: i32,
  query: Option<InsertStatement>,
  overwrite: bool
}

impl<'a> ImportDataPoints<'a> {
  pub fn new(db : &'a DatabaseConnection, resource_type: &'a str) -> Self {
    Self {
      db,
      resource_type,
      points_left: FLUSH_EVERY,
      query: None,
      overwrite: true
    }
  }

  /**
   * Flush current data and then change conflict handling
   */
  pub async fn change_overwrite(&mut self, overwrite : bool) -> Result<()> {
    if self.overwrite != overwrite {
      self.commit().await?;
      self.overwrite = overwrite;
    }

    Ok(())
  }

  fn handle_conflict(&self) -> OnConflict {
    if self.overwrite {
      OnConflict::columns(vec![
        Column::ResourceId,
        Column::ResourceType,
        Column::Date,
        Column::Kind,
      ])
      .update_columns([Column::Value])
      .to_owned()
    } else {
      OnConflict::new().do_nothing().to_owned()
    }
  }

  fn reset(&mut self) {
    tracing::trace!("Initializing new query");
    let query = Query::insert()
      .into_table(Entity)
      .columns(vec![
        Column::ResourceId,
        Column::ResourceType,
        Column::Date,
        Column::Kind,
        Column::Value,
      ]).on_conflict(self.handle_conflict())
      .to_owned();

    self.query = Some(query);
    self.points_left = FLUSH_EVERY;
  }

  pub async fn add_option(&mut self, resource_id : String, kind: Kind, at: NaiveDate, value: Option<f64>) -> anyhow::Result<()> {
    if let Some(value) = value {
      return self.add(resource_id, kind, at, value).await;
    }

    Ok(())
  }

  pub async fn add(&mut self, resource_id : String, kind: Kind, at: NaiveDate, value: f64) -> anyhow::Result<()> {
    if self.query.is_none() {
      self.reset();
    }

    let query = self.query.as_mut().unwrap();
    query.exprs(vec![
      Expr::val(resource_id).into(),
      Expr::val(self.resource_type).into(),
      Expr::val(at).into(),
      Expr::val(kind).into(),
      Expr::val(value).into()
    ])?;

    self.points_left -= 1;

    if self.points_left <= 0 {
      self.commit().await?;
    }

    Ok(())
  }

  pub async fn commit(&mut self) -> anyhow::Result<()> {
    if let Some(query) = self.query.as_ref() {
      tracing::debug!("Flushing {} points", FLUSH_EVERY - self.points_left);
      let (sql, values) = query.build(PostgresQueryBuilder);
      let statement = Statement {
        sql,
        values: Some(values),
        db_backend: DatabaseBackend::Postgres
      };

      self.db.execute(statement).await?;
      self.query = None;
    } else {
      tracing::debug!("Nothing to flush, skipping...");
    }
    Ok(())
  }
}
