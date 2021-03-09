use diesel::prelude::*;
use serde::Serialize;

#[derive(Debug, Queryable, Serialize)]
pub struct Listing {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub price: String,
}
