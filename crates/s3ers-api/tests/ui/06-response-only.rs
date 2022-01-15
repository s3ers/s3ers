use s3ers_api::s3ers_api;

s3ers_api! {
    metadata: {
        description: "Does something.",
        method: POST, // An `http::Method` constant. No imports required.
        name: "some_endpoint",
        path: "/some/endpoint/:baz",
        authentication: None,
    }

    #[derive(PartialEq)] // Make sure attributes work
    response: {
        pub flag: bool,
    }
}

fn main() {
    let res1 = Response { flag: false };
    let res2 = res1.clone();

    assert_eq!(res1, res2);
}
