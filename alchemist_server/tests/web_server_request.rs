#![feature(plugin)]
#![plugin(rocket_codegen)]
extern crate rocket;
extern crate rocket_contrib;
extern crate alchemist_server;

use alchemist_server::server_routes::*;
use rocket::Response;
use rocket::http::{Status, ContentType};
use rocket::http::Method::*;
use rocket::testing::MockRequest;

macro_rules! run_test {
        ($rocket: expr, $req:expr, $test_fn:expr) => ({
            let mut req = $req;
            $test_fn(req.dispatch_with($rocket));
        })
    }

fn rockets() -> rocket::Rocket {
    rocket::ignite().mount("/api/v0/", routes![index, package]).catch(errors![not_found])
}

#[test]
fn bad_get() {
    let rockets = rockets();

    // Try to get a url that doesn't exist
    let req = MockRequest::new(Get, "/randomURL12324").header(ContentType::JSON);
    run_test!(&rockets, req, |mut response: Response| {
        assert_eq!(response.status(), Status::NotFound);

        let body = response.body()
            .unwrap()
            .into_string()
            .unwrap();
        assert!(body.contains("error"));
        assert!(body.contains("is an invalid path!"));
    });
}

#[test]
fn good_get_on_index() {
    let rockets = rockets();

    // Checks that the index exists and returns the proper contents
    let req = MockRequest::new(Get, "/api/v0/").header(ContentType::JSON);
    run_test!(&rockets, req, |mut response: Response| {
        assert_eq!(response.status(), Status::Ok);

        let body = response.body()
            .unwrap()
            .into_string()
            .unwrap();
        assert!(body.contains("Alchemist public API server."));
    });
}

#[test]
fn good_post_on_package() {
    let rockets = rockets();
    // Checks "/package" exists and returns the correct value
    let req = MockRequest::new(Post, "/api/v0/package")
            .header(ContentType::JSON)
            .body(r#"{ "package": ["sudo"], "distro": "Arch", "client": { "name": "alchemist", "version": "0.0.4" } }"#);
    run_test!(&rockets, req, |mut response: Response| {
        assert_eq!(response.status(), Status::Ok);

        let body = response.body()
            .unwrap()
            .into_string()
            .unwrap();
        assert!(body.contains("package"))
    })
}
