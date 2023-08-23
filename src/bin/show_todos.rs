use dbfun::{models::*, establish_connection};
use diesel::prelude::*;
fn main() {
  use dbfun::schema::todos::dsl::*;
  let connection = &mut establish_connection();
  let results = todos
    .filter(completed.eq(false))
    .limit(10)
    .select(Todo::as_select())
    .load(connection)
    .expect("Error loading todos");

  println!("Displaying {} todos", results.len());
  for todo in results {
    println!("{}", todo.title);
  }
}
