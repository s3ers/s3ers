#![allow(clippy::exhaustive_structs)]

use s3ers_api::{
    s3ers_api, IncomingRequest as _, OutgoingRequest as _,
    SendAccessToken,
};

s3ers_api! {
    metadata: {
        description: "Does something.",
        method: POST,
        name: "my_endpoint",
        path: "/_matrix/foo/:bar",
        rate_limited: false,
        authentication: None,
    }

    request: {
        pub hello: String,
        #[s3ers_api(header = CONTENT_TYPE)]
        pub world: String,
        #[s3ers_api(query)]
        pub q1: String,
        #[s3ers_api(query)]
        pub q2: u32,
        #[s3ers_api(path)]
        pub bar: String,
    }

    response: {
        pub hello: String,
        #[s3ers_api(header = CONTENT_TYPE)]
        pub world: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub optional_flag: Option<bool>,
    }
}

#[test]
fn request_serde() {
    let req = Request {
        hello: "hi".to_owned(),
        world: "test".to_owned(),
        q1: "query_param_special_chars %/&@!".to_owned(),
        q2: 55,
        bar: "barVal".to_owned(),
    };

    let http_req = req
        .clone()
        .try_into_http_request::<Vec<u8>>("https://homeserver.tld", SendAccessToken::None)
        .unwrap();
    let req2 = Request::try_from_http_request(http_req).unwrap();

    assert_eq!(req.hello, req2.hello);
    assert_eq!(req.world, req2.world);
    assert_eq!(req.q1, req2.q1);
    assert_eq!(req.q2, req2.q2);
    assert_eq!(req.bar, req2.bar);
}

#[test]
fn invalid_uri_should_not_panic() {
    let req = Request {
        hello: "hi".to_owned(),
        world: "test".to_owned(),
        q1: "query_param_special_chars %/&@!".to_owned(),
        q2: 55,
        bar: "barVal".to_owned(),
    };

    let result = req.try_into_http_request::<Vec<u8>>("invalid uri", SendAccessToken::None);
    assert!(result.is_err());
}
