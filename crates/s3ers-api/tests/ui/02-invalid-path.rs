use s3ers_api::s3ers_api;

s3ers_api! {
    metadata: {
        description: "This will fail.",
        method: GET,
        name: "invalid_path",
        path: "µ/°/§/€",
        authentication: None,
    }

    request: {
        #[s3ers_api(query_map)]
        pub fields: Vec<(String, String)>,
    }

    response: {}
}

s3ers_api! {
    metadata: {
        description: "This will fail.",
        method: GET,
        name: "invalid_path",
        path: "path/to/invalid space/endpoint",
        authentication: None,
    }

    request: {
        #[s3ers_api(query_map)]
        pub fields: Vec<(String, String)>,
    }

    response: {}
}

fn main() {}
