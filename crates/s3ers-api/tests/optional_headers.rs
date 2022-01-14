use s3ers_api::s3ers_api;

s3ers_api! {
    metadata: {
        description: "Does something.",
        method: GET,
        name: "no_fields",
        path: "/_matrix/my/endpoint",
        rate_limited: false,
        authentication: None,
    }

    request: {
        #[s3ers_api(header = LOCATION)]
        pub location: Option<String>,
    }

    response: {
        #[s3ers_api(header = LOCATION)]
        pub stuff: Option<String>,
    }
}
