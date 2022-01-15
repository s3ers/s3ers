#![doc(html_favicon_url = "https://www.ruma.io/favicon.ico")]
#![doc(html_logo_url = "https://www.ruma.io/images/logo.png")]
//! Types and traits for working with the [S3](https://docs.aws.amazon.com/AmazonS3/) protocol.
//!
//! This crate re-exports things from all of the other s3ers crates so you don't
//! have to manually keep all the versions in sync.
//!
//! Which crates are re-exported can be configured through cargo features.
//!
//! > ⚠ Some details might be missing because rustdoc has trouble with re-exports so you may need
//!   to refer to other crates' documentations.
//!
//! # API features
//!
//! Depending on which parts of s3ers are relevant to you, activate the following features:
//!
//! * `s3-api` -- S3 API. You probably want to enable this.
//!
//! These features have `client`- and `server`-optimized variants that are enabled respectively
//! with the `-c` and `-s` suffixes. For example:
//!   * `s3-api-c` -- The S3 API optimized for the client side.
//!   * `s3-api-s` -- The S3 API optimized for the server side.
//!
//! # Unstable features
//!
//! By using these features, you opt out of all semver guarantees s3ers otherwise provides:
//!
//! * `unstable-exhaustive-types` -- Most types in s3ers are marked as non-exhaustive to avoid
//!   breaking changes when new fields are added in the specification. This feature compiles all
//!   types as exhaustive.
//!
//! # Common features
//!
//! These submodules are usually activated by the API features when needed:
//!
//! * `api`
//!
//! # `s3ers-client` features
//!
//! The `client` feature activates [`s3ers::client`][client], and `client-ext-s3ers-api` activates
//! `s3ers-client`s `s3ers-api` feature. All other `client-*` features activate the same feature
//! without the `client-` prefix on `s3ers-client`. See the crate's documentation for the effect of
//! these features.
//!
//! If you are viewing this on `docs.rs`, you can have a look at the feature dependencies by
//! clicking **Feature flags** in the toolbar at the top.

#![warn(missing_docs)]
#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]

#[doc(no_inline)]
pub use assign::assign;
#[doc(no_inline)]
pub use js_int::{int, uint, Int, UInt};

#[doc(inline)]
pub use s3ers_serde as serde;

pub use s3ers_serde::Outgoing;

#[cfg(feature = "client")]
#[doc(inline)]
pub use s3ers_client as client;

/// (De)serializable types for various [Matrix APIs][apis] requests and responses and abstractions
/// for them.
///
/// [apis]: https://matrix.org/docs/spec/#matrix-apis
#[cfg(feature = "api")]
pub mod api {
    pub use s3ers_api::*;

    #[cfg(feature = "s3ers-s3-api")]
    #[cfg_attr(
        docsrs,
        doc(cfg(any(
            feature = "s3-api",
            feature = "s3-api-c",
            feature = "s3-api-s"
        )))
    )]
    #[doc(inline)]
    pub use s3ers_s3_api as client;
}
