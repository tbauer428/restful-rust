#![allow(proc_macro_derive_resolution_fallback)]

use diesel;
use diesel::prelude::*;

use crate::models::Listing;

use crate::schema::listings;
use crate::schema::listings::dsl::*;

// pub fn create_post(new_post: NewPost, conn: &PgConnection) -> QueryResult<Post> {
//     diesel::insert_into(posts::table)
//         .values(&new_post)
//         .get_result(conn)
// }

pub fn show_posts(connection: &PgConnection) -> QueryResult<Vec<Listing>> {
    //posts.filter(published.eq(true))
    listings.limit(5).load::<Listing>(&*connection)
}

pub fn get_listing(listing_id: i32, connection: &PgConnection) -> QueryResult<Listing> {
    listings::table
        .find(listing_id)
        .get_result::<Listing>(connection)
}

// pub fn update_post(post_id: i32, post: Post, connection: &PgConnection) -> QueryResult<Post> {
//     diesel::update(posts::table.find(post_id))
//         .set(&post)
//         .get_result(connection)
// }

// pub fn delete_post(post_id: i32, connection: &PgConnection) -> QueryResult<usize> {
//     diesel::delete(posts::table.find(post_id)).execute(connection)
// }
