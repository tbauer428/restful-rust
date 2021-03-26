use crate::{connection::DbConn, model::Listing};
use crate::{model::NewListing, repository::*};
use rocket::http::{RawStr, Status};
use rocket_contrib::json::Json;

#[get("/listings")]
pub fn index(connection: DbConn) -> String {
    let results: Vec<Listing> = show_posts(&connection).unwrap();

    println!("Found {} posts", results.len());

    let serialized_result = serde_json::to_string(&results).unwrap().to_owned();

    return serialized_result.to_string();
}

#[post(
    "/listings/create",
    format = "application/json",
    data = "<new_listing>"
)]
pub fn create(new_listing: Json<NewListing>, connection: DbConn) -> String {
    let result: Listing = create_listing(new_listing.into_inner(), &connection).unwrap();

    let serialized_result = serde_json::to_string(&result).unwrap().to_owned();

    return serialized_result;
}

#[get("/hello/<name>")]
pub fn hello_name(name: &RawStr) -> String {
    format!("Hello, {}!", name.as_str())
}

#[get("/fail")]
pub fn just_fail() -> Status {
    Status::NotAcceptable
}

#[get("/hello/<name>/<age>/<cool>")]
pub fn hello_cool(name: String, age: u8, cool: bool) -> String {
    if cool {
        format!("You're a cool {} year old, {}!", age, name)
    } else {
        format!("{}, we need to talk about your coolness.", name)
    }
}
