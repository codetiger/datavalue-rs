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
 * - **Operator overloading**: Supports standard operators directly on DataValue
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
 *
 * ## Operator Overloading
 *
 * DataValue supports operator overloading for basic operations:
 *
 * ```rust
 * use datavalue_rs::helpers;
 *
 * // Create DataValue instances
 * let num1 = helpers::int(5);
 * let num2 = helpers::int(3);
 *
 * // Use operators directly on DataValue
 * // Note that operators consume their operands, so clone if needed
 * let sum = (num1.clone() + num2.clone()).unwrap();
 * let product = (num1.clone() * num2.clone()).unwrap();
 * let difference = (helpers::int(10) - helpers::int(4)).unwrap();
 *
 * assert_eq!(sum.as_i64(), Some(8));
 * assert_eq!(product.as_i64(), Some(15));
 * assert_eq!(difference.as_i64(), Some(6));
 *
 * // Comparison operators work directly too
 * assert!(helpers::int(10) > helpers::int(5));
 * assert!(helpers::int(3) < helpers::int(7));
 * assert!(helpers::int(5) == helpers::int(5));
 * ```
 *
 * Note that operations requiring memory allocation (like string concatenation) are not supported
 * with direct operator overloading to avoid arena lifetime complications.
 */

mod access;
mod conversion;
mod datavalue;
mod de;
mod error;
pub mod helpers;
pub mod operations;
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
