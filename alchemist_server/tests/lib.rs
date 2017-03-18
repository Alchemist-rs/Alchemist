#[cfg(tests)]
mod test {
    use super::rocket;
    use rocket::testing::MockRequest;
    use rocket::http::Method::*;
    use rocket::http::{Status, ContentType};
    use rocket::Response;
    use std::collections::HashSet;

    macro_rules! run_test {
        ($rocket: expr, $req:expr, $test_fn:expr) => ({
            let mut req = $req;
            $test_fn(req.dispatch_with($rocket));
        })
    }

    #[test]
    fn bad_get() {
        let rocket = rocket();

        // Try to get a url that doesn't exist
        let req = MockRequest::new(Get, "/randomURL12324").header(ContentType::JSON);
        run_test!(&rocket, req, |mut response: Response| {
            assert_eq!(response.status(), Status::NotFound);

            let body = response.body().unwrap().into_string().unwrap();
            assert!(body.contains("error"));
            assert!(body.contains("is an invalid path!"));
        });
    }

    #[test]
    fn post_get() {
        let rocket = rocket();

        // Checks that the index exists and returns the proper contents
        let req = MockRequest::new(Get, "/").header(ContentType::JSON);
        run_test!(&rocket, req, |mut response: Response| {
            assert_eq!(response.status(), Status::Ok);

            let body = response.body().unwrap().into_string().unwrap();
            assert!(body.contains("Alchemist public API server."));
            assert!(body.contains("version"));
        });

        // Checks "/version" exists and returns the correct value
        let req = MockRequest::new(Get, "/version").header(ContentType::JSON);
        run_test!(&rocket, req, |mut response: Response| {
            assert_eq!(response.status(), Status::Ok);

            let body = response.body().unwrap().into_string().unwrap();
            assert!(body.contains("client"));
            assert!(body.contains("server"));
        })

        // Checks "/version/[client, server]" exists and returns the correct value
        let request_hash = HashSet::new();
        request_hash.insert("client");
        request_hash.insert("server");

        for cl_sr in request_hash {
            let req = MockRequest::new(Get, format!("/version/{}", cl_sr)).header(ContentType::JSON);
            run_test!(&rocket, req, |mut response: Response| {
                assert_eq!(response.status(), Status::Ok);

                let body = response.body().unwrap().into_string().unwrap();
                assert!(body.contains(format!("{}", cl_sr)));
            })
        }

        // Checks "/package" exists and returns the correct value
        let req = MockRequest::new(Post, "/package")
            .header(ContentType::JSON)
            .body(r#"{ "package": ["sudo"], "distro": "Arch", "client": { "name": "alchemist", "version": "0.0.4" } }"#);
        run_test!(&rocket, req, |mut response: Response| {
            assert_eq!(response.status(), Status::Ok);

            let body = response.body().unwrap().into_string().unwrap();
            assert!(body.contains("package"))
        })
    }
}
