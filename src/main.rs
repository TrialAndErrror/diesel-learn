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

pub fn establish_connection() -> PgConnection {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut connection = &mut establish_connection();

    match args.len() {
        1 => {
            list(&mut connection)
        },
        2 => {
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

fn list(connection: &mut PgConnection) {
    let results = grocery::dsl::grocery
        .filter(grocery::done.eq(false))
        .limit(5)
        .select(Grocery::as_select())
        .load(connection)
        .expect("Error loading Groceries");

    println!("Displaying all ({}) groceries", results.len());
    for grocery in results {
        println!("- [{}] {}: {}", grocery.id, grocery.name, grocery.amount);
    }
}

fn create_grocery_prompts() -> (String, String) {
    let mut name = String::new();
    let mut amount = String::new();

    println!("What is the name of the grocery?");
    stdin().read_line(&mut name).unwrap();
    let name = name.trim_end(); // Remove the trailing newline

    println!("What is the amount of the grocery?");
    stdin().read_line(&mut amount).unwrap();
    let amount = amount.trim_end(); // Remove the trailing newline

    (name.to_string(), amount.to_string())
}


fn create(connection: &mut PgConnection) {
    let (name, amount)= create_grocery_prompts();

    let new_log = NewGrocery {
        name,
        amount
    };

    let inserted_row = diesel::insert_into(grocery::table)
        .values(&new_log)
        .get_result::<Grocery>(connection);

    println!("{:?}", inserted_row);
}

fn check_off_grocery_prompts() -> i32 {
    let mut grocery_id_string = String::new();

    println!("What is the ID of the grocery to check off?");
    stdin().read_line(&mut grocery_id_string).unwrap();
    grocery_id_string.trim_end().parse::<i32>().unwrap()
}

fn mark_complete(connection: &mut PgConnection) {
    list(connection);

    let grocery_id= check_off_grocery_prompts();

    let groceries = grocery::dsl::grocery
        .filter(
            grocery::done.eq(false)
                .and(grocery::id.eq(grocery_id))
        );

    let updated_row = diesel::update(groceries)
        .set((
            grocery::done.eq(true),
        ))
        .get_result::<Grocery>(connection);

    println!("{:?}", updated_row);
}
