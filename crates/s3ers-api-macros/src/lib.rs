//! A procedural macro for easily generating [s3ers-api]-compatible endpoints.
//!
//! This crate should never be used directly; instead, use it through the re-exports in s3ers-api.
//! Also note that for technical reasons, the `s3ers_api!` macro is only documented in s3ers-api,
//! not here.
//!
//! [s3ers-api]: https://gitlab.com/s3ers/s3ers/-/tree/main/crates/s3ers-api

#![recursion_limit = "256"]

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

mod api;
mod attribute;
mod auth_scheme;
mod request;
mod response;
mod util;

use api::Api;
use request::expand_derive_request;
use response::expand_derive_response;

#[proc_macro]
pub fn s3ers_api(input: TokenStream) -> TokenStream {
    let api = parse_macro_input!(input as Api);
    api.expand_all().into()
}

/// Internal helper taking care of the request-specific parts of `s3ers_api!`.
#[proc_macro_derive(Request, attributes(s3ers_api))]
pub fn derive_request(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    expand_derive_request(input)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

/// Internal helper taking care of the response-specific parts of `s3ers_api!`.
#[proc_macro_derive(Response, attributes(s3ers_api))]
pub fn derive_response(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    expand_derive_response(input)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

/// A derive macro that generates no code, but registers the s3ers_api attribute so both
/// `#[s3ers_api(...)]` and `#[cfg_attr(..., s3ers_api(...))]` are accepted on the type, its fields
/// and (in case the input is an enum) variants fields.
#[doc(hidden)]
#[proc_macro_derive(_FakeDeriveS3ersApi, attributes(s3ers_api))]
pub fn fake_derive_s3ers_api(_input: TokenStream) -> TokenStream {
    TokenStream::new()
}
