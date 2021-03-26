#![allow(proc_macro_derive_resolution_fallback)]

// unsure if I need this use, tbd...
// use diesel::prelude::*;
use crate::schema::listings;
use serde::{Deserialize, Serialize};

#[derive(Debug, Queryable, Serialize, AsChangeset, Deserialize)]
#[table_name = "listings"]
pub struct Listing {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub price: String,
}

#[derive(Insertable, Serialize, Deserialize)]
#[table_name = "listings"]
pub struct NewListing {
    pub title: String,
    pub body: String,
    pub price: String,
}
