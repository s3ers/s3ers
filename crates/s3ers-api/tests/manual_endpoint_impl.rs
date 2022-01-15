//! PUT /client/r0/directory/room/:room_alias

#![allow(clippy::exhaustive_structs)]

use std::convert::TryFrom;

use bytes::BufMut;
use http::{header::CONTENT_TYPE, method::Method};
use s3ers_api::{
    error::{FromHttpRequestError, FromHttpResponseError, IntoHttpError, SError, ServerError},
    AuthScheme, EndpointError, IncomingRequest, IncomingResponse, Metadata, OutgoingRequest,
    OutgoingResponse,
};
use s3ers_serde::Outgoing;
use serde::{Deserialize, Serialize};

/// A request to create a new room alias.
#[derive(Debug)]
pub struct Request {
    pub room_id: Box<str>,         // body
    pub room_alias: Box<str>, // path
}

impl Outgoing for Request {
    type Incoming = Self;
}

const METADATA: Metadata = Metadata {
    description: "Add an alias to a room.",
    method: Method::PUT,
    name: "create_alias",
    path: "/client/r0/directory/room/:room_alias",
    authentication: AuthScheme::None,
};

impl OutgoingRequest for Request {
    type EndpointError = SError;
    type IncomingResponse = Response;

    const METADATA: Metadata = METADATA;

    fn try_into_http_request<T: Default + BufMut>(
        self,
        base_url: &str,
    ) -> Result<http::Request<T>, IntoHttpError> {
        let url = (base_url.to_owned() + METADATA.path)
            .replace(":room_alias", &self.room_alias.to_string());

        let request_body = RequestBody { room_id: self.room_id };

        let http_request = http::Request::builder()
            .method(METADATA.method)
            .uri(url)
            .body(s3ers_serde::json_to_buf(&request_body)?)
            // this cannot fail because we don't give user-supplied data to any of the
            // builder methods
            .unwrap();

        Ok(http_request)
    }
}

impl IncomingRequest for Request {
    type EndpointError = SError;
    type OutgoingResponse = Response;

    const METADATA: Metadata = METADATA;

    fn try_from_http_request<T: AsRef<[u8]>>(
        request: http::Request<T>,
    ) -> Result<Self, FromHttpRequestError> {
        let path_segments: Vec<&str> = request.uri().path()[1..].split('/').collect();
        let room_alias = {
            let decoded =
                percent_encoding::percent_decode(path_segments[5].as_bytes()).decode_utf8()?;

            TryFrom::try_from(&*decoded)?
        };

        let request_body: RequestBody = serde_json::from_slice(request.body().as_ref())?;

        Ok(Request { room_id: request_body.room_id, room_alias })
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct RequestBody {
    room_id: Box<str>,
}

/// The response to a request to create a new room alias.
#[derive(Clone, Copy, Debug)]
pub struct Response;

impl Outgoing for Response {
    type Incoming = Self;
}

impl IncomingResponse for Response {
    type EndpointError = SError;

    fn try_from_http_response<T: AsRef<[u8]>>(
        http_response: http::Response<T>,
    ) -> Result<Self, FromHttpResponseError<SError>> {
        if http_response.status().as_u16() < 400 {
            Ok(Response)
        } else {
            Err(FromHttpResponseError::Http(ServerError::Known(
                <SError as EndpointError>::try_from_http_response(http_response)?,
            )))
        }
    }
}

impl OutgoingResponse for Response {
    fn try_into_http_response<T: Default + BufMut>(
        self,
    ) -> Result<http::Response<T>, IntoHttpError> {
        let response = http::Response::builder()
            .header(CONTENT_TYPE, "application/json")
            .body(s3ers_serde::slice_to_buf(b"{}"))
            .unwrap();

        Ok(response)
    }
}