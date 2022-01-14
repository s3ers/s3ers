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
