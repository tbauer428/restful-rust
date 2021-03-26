use crate::repository::*;
use crate::{connection::DbConn, models::Listing};
use rocket::http::{RawStr, Status};

#[get("/")]
fn index(connection: DbConn) -> String {
    let results: Vec<Listing> = show_posts(&connection).unwrap();

    println!("Found {} posts", results.len());

    let serialized_result = serde_json::to_string(&results).unwrap().to_owned();

    return serialized_result.to_string();
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
