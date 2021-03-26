#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;

pub mod connection;
pub mod controller;
pub mod model;
pub mod repository;
pub mod schema;

use controller::*;

fn main() {
    rocket::ignite()
        .manage(connection::establish_connection())
        .mount(
            "/",
            routes![index, create, hello_name, just_fail, hello_cool],
        )
        .launch();
}
