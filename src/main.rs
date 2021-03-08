#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;

use rocket::http::{RawStr, Status};

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
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
    rocket::ignite().mount("/", routes![index, hello_name, just_fail, hello_cool]).launch();
}
