#![allow(dead_code)]

mod models;
mod schema;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use models::{Grocery, NewGrocery};
use schema::grocery;
use std::env;

fn main() {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let mut connection = PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url));

    create(&mut connection);
    //finish(&mut connection);
}

fn create(connection: &mut PgConnection) {
    let new_log = NewGrocery {
        name: "New Grocery Item".to_string(),
        amount: "One Item, Please".to_string()
    };

    let inserted_row = diesel::insert_into(grocery::table)
        .values(&new_log)
        .get_result::<Grocery>(connection);

    println!("{:?}", inserted_row);
}

fn finish(connection: &mut PgConnection) {
    let groceries = grocery::dsl::grocery.filter(grocery::done.eq(false).and(grocery::id.eq(1)));

    let updated_row = diesel::update(groceries)
        .set((
            grocery::done.eq(true),
            grocery::finish_timestamp.eq(Some(chrono::Utc::now())),
        ))
        .get_result::<Grocery>(connection);

    println!("{:?}", updated_row);
}
