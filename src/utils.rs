use chrono::{Utc, NaiveDate, NaiveDateTime};

pub fn today() -> NaiveDate {
  Utc::now().naive_local().date()
}

pub fn now() -> NaiveDateTime {
  Utc::now().naive_local()
}
