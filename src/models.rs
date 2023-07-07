use diesel::prelude::*;
use serde::Serialize;

#[derive(Insertable)]
#[diesel(table_name = crate::schema::grocery)]
pub struct NewGrocery {
    pub name: String,
    pub amount: String,
}

#[derive(Debug, Queryable, Serialize, Selectable)]
#[diesel(table_name = crate::schema::grocery)]
pub struct Grocery {
    pub id: i32,
    pub name: String,
    pub amount: String,
    pub done: bool,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}
