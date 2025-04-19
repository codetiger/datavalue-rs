/*!
 * # datavalue-rs
 *
 * A high-performance, memory-efficient data structure for working with JSON data in Rust,
 * designed as a drop-in replacement for `serde_json::Value`.
 *
 * ## Key Features
 *
 * - **Arena-based allocation**: Improves cache locality and reduces memory fragmentation
 * - **Slices instead of Vectors**: Uses slice references to arena-allocated memory to reduce indirection
 * - **Zero-copy string handling**: Stores strings directly in the arena
 * - **API compatibility**: Similar API to `serde_json::Value` for easy adoption
 *
 * ## Usage
 *
 * ```rust
 * use datavalue_rs::{DataValue, Bump, from_str};
 *
 * let arena = Bump::new();
 * let json_str = r#"{"name": "John", "age": 30, "hobbies": ["reading", "coding"]}"#;
 * let value = from_str(&arena, json_str).unwrap();
 *
 * // Access values with indexing
 * println!("Name: {}", value["name"]);
 * println!("First hobby: {}", value["hobbies"][0]);
 * ```
 */

mod access;
mod conversion;
mod datavalue;
mod de;
mod error;
pub mod helpers;
mod ser;

// Re-export key types and functions for easy access
pub use bumpalo::Bump;
pub use datavalue::{DataValue, DataValueType, Number};
pub use error::{Error, Result};
pub use helpers::*;

/// Re-export of the bumpalo crate for convenient usage.
///
/// This provides access to the arena allocator functionality needed by DataValue.
pub mod bumpalo {
    pub use bumpalo::*;
}

/// Convenience module that mirrors serde_json's exports for easier migration
///
/// This allows code written for serde_json to be easily adapted for datavalue-rs.
pub mod json {
    pub use super::datavalue::DataValue as Value;
    pub use super::datavalue::DataValueType;
    pub use super::error::{Error, Result};
    pub use super::helpers::*;
    pub use super::{from_json, from_str, to_string, to_string_pretty};
}

// Standalone functions (similar to serde_json)
pub use de::{from_json, from_str};
pub use ser::{to_string, to_string_pretty};
