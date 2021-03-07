use rocket::local::blocking::Client;

#[test]
fn hello_world() {
    let client = Client::tracked(super::rocket()).unwrap();
    let response = client.get("/").dispatch();
    assert_eq!(response.into_string(), Some("Hello, world!".into()));

    let response = client.get("/hello/world").dispatch();
    assert_eq!(response.into_string(), Some("Hello, world!".into()));
}

