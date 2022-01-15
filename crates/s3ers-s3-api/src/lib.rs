#![doc(html_favicon_url = "https://www.ruma.io/favicon.ico")]
#![doc(html_logo_url = "https://www.ruma.io/images/logo.png")]
//! (De)serializable types for the [S3 API][api].
//! These types can be shared by client and server code.
//!
//! [api]: https://docs.aws.amazon.com/AmazonS3/latest/API/Welcome.html

#![warn(missing_docs)]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

pub mod error;

pub mod objects;

// pub use error::Error;

// Wrapper around `Box<str>` that cannot be used in a meaningful way outside of
// this crate. Used for string enums because their `_Custom` variant can't be
// truly private (only `#[doc(hidden)]`).
#[doc(hidden)]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct PrivOwnedStr(Box<str>);
