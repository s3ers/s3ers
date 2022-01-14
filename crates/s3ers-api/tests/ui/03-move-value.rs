// This tests that the "body" fields are moved after all other fields because they
// consume the request/response.

mod newtype_body {
    use s3ers_api::s3ers_api;

    #[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
    pub struct Foo;

    s3ers_api! {
        metadata: {
            description: "Does something.",
            method: POST,
            name: "my_endpoint",
            path: "/_matrix/foo/:bar/",
            authentication: None,
        }

        request: {
            #[s3ers_api(body)]
            pub q2: Foo,

            #[s3ers_api(path)]
            pub bar: String,

            #[s3ers_api(query)]
            pub baz: Box<str>,

            #[s3ers_api(header = CONTENT_TYPE)]
            pub world: String,
        }

        response: {
            #[s3ers_api(body)]
            pub q2: Foo,

            #[s3ers_api(header = CONTENT_TYPE)]
            pub world: String,
        }
    }
}

mod raw_body {
    use s3ers_api::s3ers_api;

    s3ers_api! {
        metadata: {
            description: "Does something.",
            method: POST,
            name: "my_endpoint",
            path: "/_matrix/foo/:bar/",
            authentication: None,
        }

        request: {
            #[s3ers_api(raw_body)]
            pub q2: Vec<u8>,

            #[s3ers_api(path)]
            pub bar: String,

            #[s3ers_api(query)]
            pub baz: Box<str>,

            #[s3ers_api(header = CONTENT_TYPE)]
            pub world: String,
        }

        response: {
            #[s3ers_api(raw_body)]
            pub q2: Vec<u8>,

            #[s3ers_api(header = CONTENT_TYPE)]
            pub world: String,
        }
    }
}

mod plain {
    use s3ers_api::s3ers_api;

    #[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
    pub struct Foo;

    s3ers_api! {
        metadata: {
            description: "Does something.",
            method: POST,
            name: "my_endpoint",
            path: "/_matrix/foo/:bar/",
            authentication: None,
        }

        request: {
            pub q2: Foo,

            pub bar: String,

            #[s3ers_api(query)]
            pub baz: Box<str>,

            #[s3ers_api(header = CONTENT_TYPE)]
            pub world: String,
        }

        response: {
            pub q2: Vec<u8>,

            #[s3ers_api(header = CONTENT_TYPE)]
            pub world: String,
        }
    }
}

fn main() {}
