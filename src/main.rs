#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;

pub mod connection;
mod controller;
pub mod models;
mod repository;
pub mod schema;

use connection::{establish_connection, PgPool, PgPooledConnection};
use diesel::prelude::*;
use models::Listing;
use rocket::http::{RawStr, Status};
use std::env;

pub fn build_db_pool() {
    establish_connection();
}

fn pg_pool_handler(pool: PgPool) -> Result<PgPooledConnection, Status> {
    pool.get().map_err(|err| Status::InternalServerError)
}

fn main() {
    rocket::ignite()
        .manage(connection::establish_connection())
        .mount("/", routes![index, hello_name, just_fail, hello_cool])
        .launch();
}
