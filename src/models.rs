use crate::schema::*;
use diesel::prelude::*;
use serde::Serialize;

#[derive(Insertable)]
#[table_name = "groceries"]
pub struct NewGrocery {
    pub name: String,
    pub amount: String,
}

#[derive(Debug, Queryable, Serialize)]
pub struct Todo {
    pub id: i32,
    pub name: String,
    pub amount: String,
    pub done: bool,
    pub finish_timestamp: Option<chrono::DateTime<chrono::Utc>>,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}
