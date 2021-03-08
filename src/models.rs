use diesel::prelude::*;

#[derive(Debug, Queryable)]
pub struct Listing {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub price: String,
}
