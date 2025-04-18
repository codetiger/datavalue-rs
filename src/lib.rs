mod access;
mod conversion;
mod datavalue;
mod de;
mod error;
pub mod helpers;
mod ser;

pub use bumpalo::Bump;
pub use datavalue::{DataValue, Number};
pub use error::{Error, Result};
pub use helpers::*;

/// Re-export of the bumpalo crate for convenient usage.
pub mod bumpalo {
    pub use bumpalo::*;
}

// Convenience module that mirrors serde_json's exports
pub mod json {
    pub use super::datavalue::DataValue as Value;
    pub use super::error::{Error, Result};
    pub use super::helpers::*;
    pub use super::{from_json, from_str, to_string};
}

// Standalone functions (similar to serde_json)
pub use de::{from_json, from_str};
pub use ser::to_string;
