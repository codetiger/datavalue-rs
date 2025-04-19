//! Serialization functionality for DataValue
//!
//! This module provides serialization capabilities for DataValue, allowing conversion
//! to JSON strings and integration with serde's serialization system.

use crate::datavalue::{DataValue, Number};
use crate::error::{Error, Result};
use serde::ser::{Serialize, SerializeMap, SerializeSeq, Serializer};

/// Converts a DataValue to a JSON string
///
/// This produces a compact representation without extra whitespace.
///
/// # Example
///
/// ```
/// # use datavalue_rs::{DataValue, Bump, helpers};
/// # let arena = Bump::new();
/// let obj = helpers::object(&arena, vec![
///     (arena.alloc_str("name"), helpers::string(&arena, "John")),
///     (arena.alloc_str("age"), helpers::int(30)),
/// ]);
///
/// let json = datavalue_rs::to_string(&obj);
/// assert_eq!(json, r#"{"name":"John","age":30}"#);
/// ```
pub fn to_string(value: &DataValue<'_>) -> String {
    format!("{}", value)
}

/// Converts a DataValue to a pretty-printed JSON string
///
/// This produces a formatted representation with indentation and line breaks
/// for improved readability.
///
/// # Example
///
/// ```
/// # use datavalue_rs::{DataValue, Bump, helpers, to_string_pretty};
/// # let arena = Bump::new();
/// let obj = helpers::object(&arena, vec![
///     (arena.alloc_str("name"), helpers::string(&arena, "John")),
///     (arena.alloc_str("age"), helpers::int(30)),
/// ]);
///
/// let json = to_string_pretty(&obj);
/// // Result will include indentation and line breaks
/// assert!(json.contains("{\n"));
/// assert!(json.contains("  \"name\""));
/// ```
pub fn to_string_pretty(value: &DataValue<'_>) -> String {
    // A simple pretty-printing implementation
    let mut result = String::new();
    to_string_pretty_internal(value, 0, &mut result);
    result
}

/// Internal helper function for pretty-printing
///
/// Recursively formats the DataValue with proper indentation.
fn to_string_pretty_internal(value: &DataValue<'_>, indent: usize, output: &mut String) {
    let indent_str = "  ".repeat(indent);

    match value {
        DataValue::Null => output.push_str("null"),
        DataValue::Bool(b) => output.push_str(if *b { "true" } else { "false" }),
        DataValue::Number(Number::Integer(i)) => output.push_str(&i.to_string()),
        DataValue::Number(Number::Float(f)) => output.push_str(&f.to_string()),
        DataValue::String(s) => {
            output.push('"');
            output.push_str(&s.replace('\"', "\\\""));
            output.push('"');
        }
        DataValue::Array(arr) => {
            if arr.is_empty() {
                output.push_str("[]");
                return;
            }

            output.push_str("[\n");
            for (i, item) in arr.iter().enumerate() {
                output.push_str(&"  ".repeat(indent + 1));
                to_string_pretty_internal(item, indent + 1, output);
                if i < arr.len() - 1 {
                    output.push(',');
                }
                output.push('\n');
            }
            output.push_str(&indent_str);
            output.push(']');
        }
        DataValue::Object(obj) => {
            if obj.is_empty() {
                output.push_str("{}");
                return;
            }

            output.push_str("{\n");
            for (i, (key, value)) in obj.iter().enumerate() {
                output.push_str(&"  ".repeat(indent + 1));
                output.push('"');
                output.push_str(&key.replace('\"', "\\\""));
                output.push_str("\": ");
                to_string_pretty_internal(value, indent + 1, output);
                if i < obj.len() - 1 {
                    output.push(',');
                }
                output.push('\n');
            }
            output.push_str(&indent_str);
            output.push('}');
        }
        DataValue::DateTime(dt) => output.push_str(&dt.to_rfc3339()),
        DataValue::Duration(dur) => output.push_str(&dur.to_string()),
    }
}

/// Implementation of serde's Serialize trait for DataValue
///
/// This allows DataValue to be used with serde's serialization framework.
impl Serialize for DataValue<'_> {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            DataValue::Null => serializer.serialize_none(),
            DataValue::Bool(b) => serializer.serialize_bool(*b),
            DataValue::Number(Number::Integer(i)) => serializer.serialize_i64(*i),
            DataValue::Number(Number::Float(f)) => serializer.serialize_f64(*f),
            DataValue::String(s) => serializer.serialize_str(s),
            DataValue::Array(arr) => {
                let mut seq = serializer.serialize_seq(Some(arr.len()))?;
                for item in *arr {
                    seq.serialize_element(item)?;
                }
                seq.end()
            }
            DataValue::Object(obj) => {
                let mut map = serializer.serialize_map(Some(obj.len()))?;
                for (key, value) in *obj {
                    map.serialize_entry(key, value)?;
                }
                map.end()
            }
            DataValue::DateTime(dt) => serializer.serialize_str(&dt.to_rfc3339()),
            DataValue::Duration(dur) => serializer.serialize_str(&dur.to_string()),
        }
    }
}

// Additional functions to write to writers
impl DataValue<'_> {
    /// Serialize to a writer
    ///
    /// Writes the compact JSON representation of this value to the given writer.
    ///
    /// # Errors
    ///
    /// Returns an error if writing to the writer fails.
    pub fn to_writer<W: std::io::Write>(&self, mut writer: W) -> Result<()> {
        let s = format!("{}", self);
        writer.write_all(s.as_bytes()).map_err(Error::from)
    }

    /// Serialize to a writer with pretty-printing
    ///
    /// Writes the pretty-printed JSON representation of this value to the given writer.
    ///
    /// # Errors
    ///
    /// Returns an error if writing to the writer fails.
    pub fn to_writer_pretty<W: std::io::Write>(&self, mut writer: W) -> Result<()> {
        let s = to_string_pretty(self);
        writer.write_all(s.as_bytes()).map_err(Error::from)
    }
}
