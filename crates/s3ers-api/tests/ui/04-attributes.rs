use s3ers_api::s3ers_api;

s3ers_api! {
    metadata: {
        description: "Does something.",
        method: POST, // An `http::Method` constant. No imports required.
        name: "some_endpoint",
        path: "/some/endpoint/:baz",
        authentication: None,
    }

    #[not_a_real_attribute_should_fail]
    request: {
        pub foo: String,
        #[s3ers_api(header = CONTENT_TYPE)]
        pub content_type: String,
        #[s3ers_api(query)]
        pub bar: String,
        #[s3ers_api(path)]
        pub baz: String,
    }

    response: {
        pub value: String,
    }
}

fn main() {}
