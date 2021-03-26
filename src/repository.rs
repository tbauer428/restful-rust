#![allow(proc_macro_derive_resolution_fallback)]

use diesel::prelude::*;
use diesel::{self, delete, update};

use crate::model::{Listing, NewListing};

use crate::schema::listings;
use crate::schema::listings::dsl::*;

pub fn create_listing(new_listing: NewListing, conn: &PgConnection) -> QueryResult<Listing> {
    diesel::insert_into(listings::table)
        .values(&new_listing)
        .get_result(conn)
}

pub fn show_posts(connection: &PgConnection) -> QueryResult<Vec<Listing>> {
    //listings.filter(price.eq("$3.00"))
    listings.limit(5).load::<Listing>(&*connection)
}

pub fn get_listing(listing_id: i32, connection: &PgConnection) -> QueryResult<Listing> {
    listings::table
        .find(listing_id)
        .get_result::<Listing>(connection)
}

pub fn update_listing(
    listing_id: i32,
    listing: Listing,
    connection: &PgConnection,
) -> QueryResult<Listing> {
    update(listings::table.find(listing_id))
        .set(&listing)
        .get_result(connection)
}

pub fn delete_listing(listing_id: i32, connection: &PgConnection) -> QueryResult<usize> {
    delete(listings::table.find(listing_id)).execute(connection)
}
