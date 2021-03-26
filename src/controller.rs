use crate::{connection::DbConn, model::Listing};
use crate::{model::NewListing, repository::*};
use rocket::http::Status;
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

#[get("/fail")]
pub fn just_fail() -> Status {
    Status::NotAcceptable
}
