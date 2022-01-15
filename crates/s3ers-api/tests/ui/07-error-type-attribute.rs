use s3ers_api::s3ers_api;

s3ers_api! {
    metadata: {
        description: "Does something.",
        method: POST, // An `http::Method` constant. No imports required.
        name: "some_endpoint",
        path: "/some/endpoint/:baz",
        authentication: None,
    }

    request: {}

    response: {}

    #[derive(Default)]
    error: s3ers_api::error::SError
}

fn main() {}
