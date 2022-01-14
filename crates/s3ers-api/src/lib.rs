//! Core types used to define the requests and responses for each endpoint in the various
//! [S3 API specifications][apis].
//!
//! When implementing a new S3 API, each endpoint has a request type which implements
//! `Endpoint`, and a response type connected via an associated type.
//!
//! An implementation of `Endpoint` contains all the information about the HTTP method, the path and
//! input parameters for requests, and the structure of a successful response.
//! Such types can then be used by client code to make requests, and by server code to fulfill
//! those requests.
//!
//! [apis]: https://docs.aws.amazon.com/AmazonS3/latest/API/Welcome.html

#![warn(missing_docs)]

#[cfg(not(all(feature = "client", feature = "server")))]
compile_error!("s3ers_api's Cargo features only exist as a workaround are not meant to be disabled");

use std::error::Error as StdError;

use bytes::BufMut;
use http::Method;

/// Generates a `s3ers_api::Endpoint` from a concise definition.
///
/// The macro expects the following structure as input:
///
/// ```text
/// s3ers_api! {
///     metadata: {
///         description: &'static str,
///         method: http::Method,
///         name: &'static str,
///         path: &'static str,
///         authentication: s3ers_api::AuthScheme,
///     }
///
///     request: {
///         // Struct fields for each piece of data required
///         // to make a request to this API endpoint.
///     }
///
///     response: {
///         // Struct fields for each piece of data expected
///         // in the response from this API endpoint.
///     }
///
///     // The error returned when a response fails, defaults to `S3Error`.
///     error: path::to::Error
/// }
/// ```
///
/// This will generate a `s3ers_api::Metadata` value to be used for the `s3ers_api::Endpoint`'s
/// associated constant, single `Request` and `Response` structs, and the necessary trait
/// implementations to convert the request into a `http::Request` and to create a response from
/// a `http::Response` and vice versa.
///
/// The details of each of the three sections of the macros are documented below.
///
/// ## Metadata
///
/// * `description`: A short description of what the endpoint does.
/// * `method`: The HTTP method used for requests to the endpoint. It's not necessary to import
///   `http::Method`'s associated constants. Just write the value as if it was imported, e.g.
///   `GET`.
/// * `name`: A unique name for the endpoint. Generally this will be the same as the containing
///   module.
/// * `path`: The path component of the URL for the endpoint, e.g. "/foo/bar". Components of
///   the path that are parameterized can indicate a variable by using a Rust identifier
///   prefixed with a colon, e.g. `/foo/:some_parameter`. A corresponding query string
///   parameter will be expected in the request struct (see below for details).
/// * `authentication`: What authentication scheme the endpoint uses.
///
/// ## Request
///
/// The request block contains normal struct field definitions. Doc comments and attributes are
/// allowed as normal. There are also a few special attributes available to control how the
/// struct is converted into an `http::Request`:
///
/// * `#[s3ers_api(header = HEADER_NAME)]`: Fields with this attribute will be treated as HTTP
///   headers on the request. The value must implement `AsRef<str>`. Generally this is a
///   `String`. The attribute value shown above as `HEADER_NAME` must be a header name constant
///   from `http::header`, e.g. `CONTENT_TYPE`.
/// * `#[s3ers_api(path)]`: Fields with this attribute will be inserted into the matching path
///   component of the request URL.
/// * `#[s3ers_api(query)]`: Fields with this attribute will be inserting into the URL's query
///   string.
/// * `#[s3ers_api(query_map)]`: Instead of individual query fields, one query_map field, of any
///   type that implements `IntoIterator<Item = (String, String)>` (e.g. `HashMap<String,
///   String>`, can be used for cases where an endpoint supports arbitrary query parameters.
///
/// Any field that does not include one of these attributes will be part of the request's XML
/// body.
///
/// ## Response
///
/// Like the request block, the response block consists of normal struct field definitions.
/// Doc comments and attributes are allowed as normal.
/// There is also a special attribute available to control how the struct is created from a
/// `http::Request`:
///
/// * `#[s3ers_api(header = HEADER_NAME)]`: Fields with this attribute will be treated as HTTP
///   headers on the response. The value must implement `AsRef<str>`. Generally this is a
///   `String`. The attribute value shown above as `HEADER_NAME` must be a header name constant
///   from `http::header`, e.g. `CONTENT_TYPE`.
///
/// Any field that does not include the above attribute will be expected in the response's XML
/// body.
///
/// ## Newtype bodies
///
/// Both the request and response block also support "newtype bodies" by using the
/// `#[s3ers_api(body)]` attribute on a field. If present on a field, the entire request or
/// response body will be treated as the value of the field. This allows you to treat the
/// entire request or response body as a specific type, rather than a XML object with named
/// fields. Only one field in each struct can be marked with this attribute. It is an error to
/// have a newtype body field and normal body fields within the same struct.
///
/// There is another kind of newtype body that is enabled with `#[s3ers_api(raw_body)]`. It is
/// used for endpoints in which the request or response body can be arbitrary bytes instead of
/// a XML object. A field with `#[s3ers_api(raw_body)]` needs to have the type `Vec<u8>`.
///
/// # Examples
///
/// ```
/// pub mod some_endpoint {
///     use s3ers_api_macros::s3ers_api;
///
///     s3ers_api! {
///         metadata: {
///             description: "Does something.",
///             method: POST,
///             name: "some_endpoint",
///             path: "/some/endpoint/:baz",
///             authentication: None,
///         }
///
///         request: {
///             pub foo: String,
///
///             #[s3ers_api(header = CONTENT_TYPE)]
///             pub content_type: String,
///
///             #[s3ers_api(query)]
///             pub bar: String,
///
///             #[s3ers_api(path)]
///             pub baz: String,
///         }
///
///         response: {
///             #[s3ers_api(header = CONTENT_TYPE)]
///             pub content_type: String,
///
///             pub value: String,
///         }
///     }
/// }
///
/// pub mod newtype_body_endpoint {
///     use s3ers_api_macros::s3ers_api;
///     use serde::{Deserialize, Serialize};
///
///     #[derive(Clone, Debug, Deserialize, Serialize)]
///     pub struct MyCustomType {
///         pub foo: String,
///     }
///
///     s3ers_api! {
///         metadata: {
///             description: "Does something.",
///             method: PUT,
///             name: "newtype_body_endpoint",
///             path: "/some/newtype/body/endpoint",
///             authentication: None,
///         }
///
///         request: {
///             #[s3ers_api(raw_body)]
///             pub file: &'a [u8],
///         }
///
///         response: {
///             #[s3ers_api(body)]
///             pub my_custom_type: MyCustomType,
///         }
///     }
/// }
/// ```
pub use s3ers_api_macros::s3ers_api;

pub mod error;
/// This module is used to support the generated code from s3ers-api-macros.
/// It is not considered part of s3ers-api's public API.
#[doc(hidden)]
pub mod exports {
    pub use bytes;
    pub use http;
    pub use percent_encoding;
    pub use quick_xml;
    pub use s3ers_api_macros;
    pub use s3ers_serde;
    pub use serde;
}

use error::{FromHttpRequestError, FromHttpResponseError, IntoHttpError};

/// An enum to control whether an access token should be added to outgoing requests
#[derive(Clone, Copy, Debug)]
#[allow(clippy::exhaustive_enums)]
pub enum SendAccessToken<'a> {
    /// Add the given access token to the request only if the `METADATA` on the request requires
    /// it.
    IfRequired(&'a str),

    /// Always add the access token.
    Always(&'a str),

    /// Don't add an access token.
    ///
    /// This will lead to an error if the request endpoint requires authentication
    None,
}

impl<'a> SendAccessToken<'a> {
    /// Get the access token for an endpoint that requires one.
    ///
    /// Returns `Some(_)` if `self` contains an access token.
    pub fn get_required_for_endpoint(self) -> Option<&'a str> {
        match self {
            Self::IfRequired(tok) | Self::Always(tok) => Some(tok),
            Self::None => None,
        }
    }

    /// Get the access token for an endpoint that should not require one.
    ///
    /// Returns `Some(_)` only if `self` is `SendAccessToken::Always(_)`.
    pub fn get_not_required_for_endpoint(self) -> Option<&'a str> {
        match self {
            Self::Always(tok) => Some(tok),
            Self::IfRequired(_) | Self::None => None,
        }
    }
}

/// A request type for a S3 API endpoint, used for sending requests.
pub trait OutgoingRequest: Sized {
    /// A type capturing the expected error conditions the server can return.
    type EndpointError: EndpointError;

    /// Response type returned when the request is successful.
    type IncomingResponse: IncomingResponse<EndpointError = Self::EndpointError>;

    /// Metadata about the endpoint.
    const METADATA: Metadata;

    /// Tries to convert this request into an `http::Request`.
    ///
    /// This method should only fail when called on endpoints that requires authentication. It may
    /// also fail with a serialization error in case of bugs in s3ers though.
    ///
    /// The endpoints path will be appended to the given `base_url`, for example
    /// `https://s3.aws.amazon.com`. Since all paths begin with a slash, it is not necessary for the
    /// `base_url` to have a trailing slash. If it has one however, it will be ignored.
    fn try_into_http_request<T: Default + BufMut>(
        self,
        base_url: &str,
        access_token: SendAccessToken<'_>,
    ) -> Result<http::Request<T>, IntoHttpError>;
}

/// A response type for a S3 API endpoint, used for receiving responses.
pub trait IncomingResponse: Sized {
    /// A type capturing the expected error conditions the server can return.
    type EndpointError: EndpointError;

    /// Tries to convert the given `http::Response` into this response type.
    fn try_from_http_response<T: AsRef<[u8]>>(
        response: http::Response<T>,
    ) -> Result<Self, FromHttpResponseError<Self::EndpointError>>;
}

/// A request type for a S3 API endpoint, used for receiving requests.
pub trait IncomingRequest: Sized {
    /// A type capturing the error conditions that can be returned in the response.
    type EndpointError: EndpointError;

    /// Response type to return when the request is successful.
    type OutgoingResponse: OutgoingResponse;

    /// Metadata about the endpoint.
    const METADATA: Metadata;

    /// Tries to turn the given `http::Request` into this request type.
    fn try_from_http_request<T: AsRef<[u8]>>(
        req: http::Request<T>,
    ) -> Result<Self, FromHttpRequestError>;
}

/// A request type for a S3 API endpoint, used for sending responses.
pub trait OutgoingResponse {
    /// Tries to convert this response into an `http::Response`.
    ///
    /// This method should only fail when when invalid header values are specified. It may also
    /// fail with a serialization error in case of bugs in s3ers though.
    fn try_into_http_response<T: Default + BufMut>(
        self,
    ) -> Result<http::Response<T>, IntoHttpError>;
}

/// Gives users the ability to define their own serializable / deserializable errors.
pub trait EndpointError:
    OutgoingResponse + StdError + Sized + Send + 'static
{
    /// Tries to construct `Self` from an `http::Response`.
    ///
    /// This will always return `Err` variant when no `error` field is defined in
    /// the `s3ers_api` macro.
    fn try_from_http_response<T: AsRef<[u8]>>(
        response: http::Response<T>,
    ) -> Result<Self, error::DeserializationError>;
}

/// Marker trait for requests that don't require authentication, for the client side.
pub trait OutgoingNoneAuthRequest: OutgoingRequest {}

/// Marker trait for requests that don't require authentication, for the server side.
pub trait IncomingNoneAuthRequest: IncomingRequest {}

/// Authentication scheme used by the endpoint.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[allow(clippy::exhaustive_enums)]
pub enum AuthScheme {
    /// No authentication is performed.
    None,

    /// Authentication is performed by including an access token in the `Authentication` http
    /// header.
    ///
    /// It is recommended to use the header over the query parameter.
    AwsSignatureV4Header,

    /// Authentication is performed by setting the `access_token` query parameter.
    AwsSignatureV4QueryParams,
}

/// Metadata about an API endpoint.
#[derive(Clone, Debug)]
#[allow(clippy::exhaustive_structs)]
pub struct Metadata {
    /// A human-readable description of the endpoint.
    pub description: &'static str,

    /// The HTTP method used by this endpoint.
    pub method: Method,

    /// A unique identifier for this endpoint.
    pub name: &'static str,

    /// The path of this endpoint's URL, with variable names where path parameters should be filled
    /// in during a request.
    pub path: &'static str,

    /// What authentication scheme the server uses for this endpoint.
    pub authentication: AuthScheme,
}
