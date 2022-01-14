//! This module contains types for all kinds of errors that can occur when
//! converting between http requests / responses and s3ers's representation of
//! S3 API requests / responses.

use std::{error::Error as StdError, fmt};

use bytes::BufMut;
use http;
use quick_xml::de::from_slice as from_xml_slice;
use s3ers_serde::XmlValue;
use thiserror::Error;

use crate::{EndpointError, OutgoingResponse};

// TODO: prevent users from using this.
// The problem is that we can't easily make a generic error type. Maybe we can
// keep this, but set body to `String`.

/// A general-purpose S3 error type consisting of an HTTP status code and a XML body.
///
/// Note that individual `s3ers-*-api` crates may provide more specific error types.
#[allow(clippy::exhaustive_structs)]
#[derive(Clone, Debug)]
pub struct S3Error {
    /// The http response's status code.
    pub status_code: http::StatusCode,

    /// The http response's body.
    pub body: XmlValue,
}

impl fmt::Display for S3Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}] ", self.status_code.as_u16())?;
        fmt::Display::fmt(&self.body, f)
    }
}

impl StdError for S3Error {}

impl OutgoingResponse for S3Error {
    fn try_into_http_response<T: Default + BufMut>(
        self,
    ) -> Result<http::Response<T>, IntoHttpError> {
        http::Response::builder()
            .header(http::header::CONTENT_TYPE, "application/xml")
            .status(self.status_code)
            .body(s3ers_serde::xml_to_buf(&self.body).expect("TODO"))
            .map_err(Into::into)
    }
}

impl EndpointError for S3Error {
    fn try_from_http_response<T: AsRef<[u8]>>(
        response: http::Response<T>,
    ) -> Result<Self, DeserializationError> {
        Ok(Self {
            status_code: response.status(),
            body: from_xml_slice(response.body().as_ref()).expect("TODO"),
        })
    }
}

/// An error when converting one of s3ers's endpoint-specific request or response
/// types to the corresponding http type.
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum IntoHttpError {
    /// Tried to create an authentication request without an access token.
    #[error(
        "This endpoint has to be converted to http::Request using \
         try_into_authenticated_http_request"
    )]
    NeedsAuthentication,

    /// XML serialization failed.
    #[error("XML serialization failed: {0}")]
    Xml(#[from] quick_xml::Error),

    /// Query parameter serialization failed.
    #[error("Query parameter serialization failed: {0}")]
    Query(#[from] s3ers_serde::urlencoded::ser::Error),

    /// Header serialization failed.
    #[error("Header serialization failed: {0}")]
    Header(#[from] http::header::InvalidHeaderValue),

    /// HTTP request construction failed.
    #[error("HTTP request construction failed: {0}")]
    Http(#[from] http::Error),
}

/// An error when converting a http request to one of s3ers's endpoint-specific request types.
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum FromHttpRequestError {
    /// Deserialization failed
    #[error("deserialization failed: {0}")]
    Deserialization(DeserializationError),

    /// HTTP method mismatch
    #[error("http method mismatch: expected {expected}, received: {received}")]
    MethodMismatch {
        /// expected http method
        expected: http::method::Method,
        /// received http method
        received: http::method::Method,
    },
}

impl<T> From<T> for FromHttpRequestError
where
    T: Into<DeserializationError>,
{
    fn from(err: T) -> Self {
        Self::Deserialization(err.into())
    }
}

/// An error when converting a http response to one of s3ers's endpoint-specific response types.
#[derive(Debug)]
#[non_exhaustive]
pub enum FromHttpResponseError<E> {
    /// Deserialization failed
    Deserialization(DeserializationError),

    /// The server returned a non-success status
    Http(ServerError<E>),
}

impl<E: fmt::Display> fmt::Display for FromHttpResponseError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Deserialization(err) => {
                write!(f, "deserialization failed: {}", err)
            }
            Self::Http(err) => {
                write!(f, "the server returned an error: {}", err)
            }
        }
    }
}

impl<E> From<ServerError<E>> for FromHttpResponseError<E> {
    fn from(err: ServerError<E>) -> Self {
        Self::Http(err)
    }
}

impl<E, T> From<T> for FromHttpResponseError<E>
where
    T: Into<DeserializationError>,
{
    fn from(err: T) -> Self {
        Self::Deserialization(err.into())
    }
}

impl<E: StdError> StdError for FromHttpResponseError<E> {}

/// An error was reported by the server (HTTP status code 4xx or 5xx)
#[derive(Debug)]
#[allow(clippy::exhaustive_enums)]
pub enum ServerError<E> {
    /// An error that is expected to happen under certain circumstances and
    /// that has a well-defined structure
    Known(E),

    /// An error of unexpected type of structure
    Unknown(DeserializationError),
}

impl<E: fmt::Display> fmt::Display for ServerError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ServerError::Known(e) => fmt::Display::fmt(e, f),
            ServerError::Unknown(res_err) => fmt::Display::fmt(res_err, f),
        }
    }
}

impl<E: StdError> StdError for ServerError<E> {}

/// An error when converting a http request / response to one of s3ers's endpoint-specific request /
/// response types.
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum DeserializationError {
    /// Encountered invalid UTF-8.
    #[error("{0}")]
    Utf8(#[from] std::str::Utf8Error),

    /// XML deserialization failed.
    #[error("{0}")]
    Xml(#[from] quick_xml::Error),

    /// Query parameter deserialization failed.
    #[error("{0}")]
    Query(#[from] s3ers_serde::urlencoded::de::Error),

    // TODO
    // Got an invalid identifier.
    // #[error("{0}")]
    // Ident(#[from] s3ers_identifiers::Error),
    /// Header value deserialization failed.
    #[error("{0}")]
    Header(#[from] HeaderDeserializationError),
}

impl From<std::convert::Infallible> for DeserializationError {
    fn from(err: std::convert::Infallible) -> Self {
        match err {}
    }
}

impl From<http::header::ToStrError> for DeserializationError {
    fn from(err: http::header::ToStrError) -> Self {
        Self::Header(HeaderDeserializationError::ToStrError(err))
    }
}

/// An error with the http headers.
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum HeaderDeserializationError {
    /// Failed to convert `http::header::HeaderValue` to `str`.
    #[error("{0}")]
    ToStrError(http::header::ToStrError),

    /// The given required header is missing.
    #[error("Missing header `{0}`")]
    MissingHeader(String),
}
