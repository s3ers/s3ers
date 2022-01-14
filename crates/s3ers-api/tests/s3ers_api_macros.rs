#![allow(clippy::exhaustive_structs)]

pub mod some_endpoint {
    use s3ers_api::s3ers_api;
    use s3ers_serde::Raw;

    s3ers_api! {
        metadata: {
            description: "Does something.",
            method: POST, // An `http::Method` constant. No imports required.
            name: "some_endpoint",
            path: "/_matrix/some/endpoint/:user",

            #[cfg(all())]
            rate_limited: true,
            #[cfg(any())]
            rate_limited: false,

            #[cfg(all())]
            authentication: AccessToken,
            #[cfg(any())]
            authentication: None,
        }

        request: {
            // With no attribute on the field, it will be put into the body of the request.
            pub a_field: String,

            // This value will be put into the "Content-Type" HTTP header.
            #[s3ers_api(header = CONTENT_TYPE)]
            pub content_type: String,

            // This value will be put into the query string of the request's URL.
            #[s3ers_api(query)]
            pub bar: String,

            // This value will be inserted into the request's URL in place of the
            // ":user" path component.
            #[s3ers_api(path)]
            pub user: Box<str>,
        }

        response: {
            // This value will be extracted from the "Content-Type" HTTP header.
            #[s3ers_api(header = CONTENT_TYPE)]
            pub content_type: String,

            // With no attribute on the field, it will be extracted from the body of the response.
            pub value: String,

            // You can use serde attributes on any kind of field
            #[serde(skip_serializing_if = "Option::is_none")]
            pub optional_flag: Option<bool>,

            // Use `Raw` instead of the actual event to allow additional fields to be sent...
            pub event: Raw<String>,

            // ... and to allow unknown events when the endpoint deals with event collections.
            pub list_of_events: Vec<Raw<String>>,
        }
    }
}

pub mod newtype_body_endpoint {
    use s3ers_api::s3ers_api;

    #[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
    pub struct MyCustomType {
        pub a_field: String,
    }

    s3ers_api! {
        metadata: {
            description: "Does something.",
            method: PUT,
            name: "newtype_body_endpoint",
            path: "/_matrix/some/newtype/body/endpoint",
            rate_limited: false,
            authentication: None,
        }

        request: {
            #[s3ers_api(body)]
            pub list_of_custom_things: Vec<MyCustomType>,
        }

        response: {
            #[s3ers_api(body)]
            pub my_custom_thing: MyCustomType,
        }
    }
}

pub mod raw_body_endpoint {
    use s3ers_api::s3ers_api;

    #[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
    pub struct MyCustomType {
        pub a_field: String,
    }

    s3ers_api! {
        metadata: {
            description: "Does something.",
            method: PUT,
            name: "newtype_body_endpoint",
            path: "/_matrix/some/newtype/body/endpoint",
            rate_limited: false,
            authentication: None,
        }

        request: {
            #[s3ers_api(raw_body)]
            pub file: &'a [u8],
        }

        response: {
            #[s3ers_api(raw_body)]
            pub file: Vec<u8>,
        }
    }
}

pub mod query_map_endpoint {
    use s3ers_api::s3ers_api;

    s3ers_api! {
        metadata: {
            description: "Does something.",
            method: GET,
            name: "newtype_body_endpoint",
            path: "/_matrix/some/query/map/endpoint",
            rate_limited: false,
            authentication: None,
        }

        request: {
            #[s3ers_api(query_map)]
            pub fields: Vec<(String, String)>,
        }

        response: {}
    }
}
