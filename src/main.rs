#![allow(dead_code)]

mod models;
mod schema;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use models::{Grocery, NewGrocery};
use schema::grocery;
use std::env;
use std::io::{stdin};

fn help() {
    println!("Enter 'n' for new record or 'x' for checking off a record");
}


fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => {
            let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
            let mut connection = PgConnection::establish(&database_url)
                .expect(&format!("Error connecting to {}", database_url));

            list(&mut connection)
        },
        2 => {
            let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
            let mut connection = PgConnection::establish(&database_url)
                .expect(&format!("Error connecting to {}", database_url));

            match args[1].parse() {
                Ok('n') => create(&mut connection),
                Ok('x') => mark_complete(&mut connection),
                _ => println!("Invalid command passed. Please try 'n' or 'x'"),
            }
        },
        _ => {
            // show a help message
            help();
        }
    }
}

fn list(_connection: &mut PgConnection) {
    println!("Please enter a command: 'n' for new record or 'x' to check off a record");
}


fn create(connection: &mut PgConnection) {
    let mut name = String::new();
    let mut amount = String::new();

    println!("What is the name of the grocery?");
    stdin().read_line(&mut name).unwrap();
    let name = name.trim_end(); // Remove the trailing newline

    println!("What is the amount of the grocery?");
    stdin().read_line(&mut amount).unwrap();
    let amount = amount.trim_end(); // Remove the trailing newline

    let new_log = NewGrocery {
        name: name.to_string(),
        amount: amount.to_string()
    };

    let inserted_row = diesel::insert_into(grocery::table)
        .values(&new_log)
        .get_result::<Grocery>(connection);

    println!("{:?}", inserted_row);
}

fn mark_complete(connection: &mut PgConnection) {
    let mut grocery_id_string = String::new();

    println!("What is the ID of the grocery?");
    stdin().read_line(&mut grocery_id_string).unwrap();
    let grocery_id_string = grocery_id_string.trim_end(); // Remove the trailing newline

    let grocery_id = grocery_id_string.parse::<i32>().unwrap();

    let groceries = grocery::dsl::grocery.filter(grocery::done.eq(false).and(grocery::id.eq(grocery_id)));

    let updated_row = diesel::update(groceries)
        .set((
            grocery::done.eq(true),
        ))
        .get_result::<Grocery>(connection);

    println!("{:?}", updated_row);
}
