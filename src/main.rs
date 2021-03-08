#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;

pub mod models;
pub mod schema;

use self::models::*;
use crate::schema::*;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use rocket::http::{RawStr, Status};
use rocket::response::content;
use std::env;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

#[get("/")]
fn index() -> content::Json<&'static str> {
    content::Json("{'name': 'keyboard', 'price': '19.99'}")
}

#[get("/hello/<name>")]
fn hello_name(name: &RawStr) -> String {
    format!("Hello, {}!", name.as_str())
}

#[get("/fail")]
fn just_fail() -> Status {
    Status::NotAcceptable
}

#[get("/hello/<name>/<age>/<cool>")]
fn hello_cool(name: String, age: u8, cool: bool) -> String {
    if cool {
        format!("You're a cool {} year old, {}!", age, name)
    } else {
        format!("{}, we need to talk about your coolness.", name)
    }
}

fn main() {
    // rocket::ignite()
    //     .mount("/", routes![index, hello_name, just_fail, hello_cool])
    //     .launch();
    let connection = establish_connection();

    use self::schema::listings::dsl::*;

    let results = listings
        .filter(title.eq("keyboard"))
        .limit(5)
        .load::<Listing>(&connection)
        .expect("Error loading listings");

    println!("Displaying {} posts", results.len());
    for listing in results {
        println!("{}", listing.title);
        println!("-----------\n");
        println!("{}", listing.price);
    }
}
