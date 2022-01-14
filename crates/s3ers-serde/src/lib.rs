use serde::{Deserialize, Serialize};
use std::fmt;

pub use self::buf::{slice_to_buf, xml_to_buf};

mod buf;
pub mod urlencoded;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum XmlValue {
    String(String),
}

impl fmt::Display for XmlValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt("", f)
    }
}

/// A type that can be sent to another party that understands the s3 protocol.
///
/// If any of the fields of `Self` don't implement serde's `Deserialize`, you can derive this trait
/// to generate a corresponding 'Incoming' type that supports deserialization. This is useful for
/// things like s3ers_events' `EventResult` type. For more details, see the
/// [derive macro's documentation][doc].
///
/// [doc]: derive.Outgoing.html
// TODO: Better explain how this trait relates to serde's traits
pub trait Outgoing {
    /// The 'Incoming' variant of `Self`.
    type Incoming;
}
