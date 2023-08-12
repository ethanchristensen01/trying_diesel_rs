use diesel::prelude::*;
use chrono::NaiveDate;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::todos)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Todo {
  pub id: i32,
  pub title: String,
  pub completed: bool,
  pub due_date: Option<NaiveDate>
}