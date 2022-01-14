#![allow(clippy::exhaustive_structs)]

#[derive(Copy, Clone, Debug, s3ers_serde::Outgoing, serde::Serialize)]
pub struct OtherThing<'t> {
    pub some: &'t str,
    pub t: &'t [u8],
}

mod empty_response {
    use s3ers_api::s3ers_api;

    s3ers_api! {
        metadata: {
            description: "Add an alias to a room.",
            method: PUT,
            name: "create_alias",
            path: "/_matrix/client/r0/directory/room/:room_alias",
            authentication: AwsSignatureV4Header,
        }

        request: {
            /// The room alias to set.
            #[s3ers_api(path)]
            pub room_alias: &'a str,

            /// The room ID to set.
            pub room_id: &'a str,
        }

        response: {}
    }
}

mod nested_types {
    use s3ers_api::s3ers_api;

    s3ers_api! {
        metadata: {
            description: "Add an alias to a room.",
            method: PUT,
            name: "create_alias",
            path: "/_matrix/client/r0/directory/room/:room_alias",
            authentication: AwsSignatureV4Header,
        }

        request: {
            /// The room alias to set.
            pub room_alias: &'a [Option<&'a String>],

            /// The room ID to set.
            pub room_id: &'b [Option<Option<&'a String>>],
        }

        response: {}
    }
}

mod full_request_response {
    use s3ers_api::s3ers_api;

    use super::{IncomingOtherThing, OtherThing};

    s3ers_api! {
        metadata: {
            description: "Does something.",
            method: POST,
            name: "no_fields",
            path: "/_matrix/my/endpoint/:thing",
            authentication: None,
        }

        request: {
            #[s3ers_api(query)]
            pub abc: &'a str,
            #[s3ers_api(path)]
            pub thing: &'a str,
            #[s3ers_api(header = CONTENT_TYPE)]
            pub stuff: &'a str,
            pub more: OtherThing<'t>,
        }

        response: {
            #[s3ers_api(body)]
            pub thing: Vec<String>,
            #[s3ers_api(header = CONTENT_TYPE)]
            pub stuff: String,
        }
    }
}

mod full_request_response_with_query_map {
    use s3ers_api::s3ers_api;

    s3ers_api! {
        metadata: {
            description: "Does something.",
            method: GET,
            name: "no_fields",
            path: "/_matrix/my/endpoint/:thing",
            authentication: None,
        }

        request: {
            #[s3ers_api(query_map)]
            // pub abc: &'a [(&'a str, &'a str)], // TODO handle this use case
            pub abc: Vec<(String, String)>,
            #[s3ers_api(path)]
            pub thing: &'a str,
            #[s3ers_api(header = CONTENT_TYPE)]
            pub stuff: &'a str,
        }

        response: {
            #[s3ers_api(body)]
            pub thing: String,
            #[s3ers_api(header = CONTENT_TYPE)]
            pub stuff: String,
        }
    }
}

mod query_fields {
    use s3ers_api::s3ers_api;

    s3ers_api! {
        metadata: {
            description: "Get the list of rooms in this homeserver's public directory.",
            method: GET,
            name: "get_public_rooms",
            path: "/_matrix/client/r0/publicRooms",
            authentication: None,
        }

        request: {
            /// Limit for the number of results to return.
            #[serde(skip_serializing_if = "Option::is_none")]
            #[s3ers_api(query)]
            pub limit: Option<usize>,

            /// Pagination token from a previous request.
            #[serde(skip_serializing_if = "Option::is_none")]
            #[s3ers_api(query)]
            pub since: Option<&'a str>,

            /// The server to fetch the public room lists from.
            ///
            /// `None` means the server this request is sent to.
            #[serde(skip_serializing_if = "Option::is_none")]
            #[s3ers_api(query)]
            pub server: Option<&'a str>,
        }

        response: {}
    }
}
