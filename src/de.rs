//! Deserialization functionality for DataValue
//!
//! This module provides functions to deserialize JSON strings into DataValue instances
//! and to convert serde_json::Value structures to DataValue.

use crate::datavalue::{DataValue, Number};
use crate::error::{Error, Result};
use bumpalo::Bump;
use serde::de::Deserializer;
use std::io::Read;

/// Parse a JSON string into a DataValue using serde_json for parsing
///
/// This function uses serde_json to parse the JSON string, then converts
/// the resulting serde_json::Value into a DataValue.
///
/// # Arguments
///
/// * `arena` - The arena allocator to store strings, arrays, and objects
/// * `s` - The JSON string to parse
///
/// # Returns
///
/// Result containing the parsed DataValue or an error
///
/// # Example
///
/// ```
/// # use datavalue_rs::{DataValue, Bump, from_str};
/// let arena = Bump::new();
/// let json = r#"{"name": "John", "age": 30}"#;
///
/// let value = from_str(&arena, json).unwrap();
/// assert_eq!(value["name"].as_str(), Some("John"));
/// assert_eq!(value["age"].as_i64(), Some(30));
/// ```
pub fn from_str<'a>(arena: &'a Bump, s: &str) -> Result<DataValue<'a>> {
    // Parse the string using serde_json
    let json_value: serde_json::Value = serde_json::from_str(s)?;

    // Convert the serde_json::Value to DataValue
    from_json(arena, &json_value)
}

/// Convert a serde_json::Value into a DataValue
///
/// This function recursively converts a serde_json::Value into a DataValue,
/// allocating strings, arrays, and objects in the provided arena.
///
/// # Arguments
///
/// * `arena` - The arena allocator to store strings, arrays, and objects
/// * `json` - The serde_json::Value to convert
///
/// # Returns
///
/// Result containing the converted DataValue or an error
///
/// # Example
///
/// ```
/// # use datavalue_rs::{DataValue, Bump, from_json};
/// # use serde_json::json;
/// let arena = Bump::new();
///
/// // Create a serde_json::Value
/// let json_value = json!({
///     "name": "John",
///     "age": 30,
///     "hobbies": ["reading", "coding"]
/// });
///
/// let value = from_json(&arena, &json_value).unwrap();
/// assert_eq!(value["name"].as_str(), Some("John"));
/// assert_eq!(value["hobbies"][0].as_str(), Some("reading"));
/// ```
pub fn from_json<'a>(arena: &'a Bump, json: &serde_json::Value) -> Result<DataValue<'a>> {
    match json {
        serde_json::Value::Null => Ok(DataValue::Null),
        serde_json::Value::Bool(b) => Ok(DataValue::Bool(*b)),
        serde_json::Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                Ok(DataValue::Number(Number::Integer(i)))
            } else if let Some(f) = n.as_f64() {
                Ok(DataValue::Number(Number::Float(f)))
            } else {
                Err(Error::syntax("Unsupported number type".to_string()))
            }
        }
        serde_json::Value::String(s) => {
            let s_ref = arena.alloc_str(s);
            Ok(DataValue::String(s_ref))
        }
        serde_json::Value::Array(arr) => {
            let mut values = Vec::with_capacity(arr.len());
            for item in arr {
                values.push(from_json(arena, item)?);
            }

            // Allocate the values in the arena
            let values_slice = arena.alloc_slice_clone(&values);
            Ok(DataValue::Array(values_slice))
        }
        serde_json::Value::Object(map) => {
            // Create the entries with explicit type
            let mut entries: Vec<(&'a str, DataValue<'a>)> = Vec::with_capacity(map.len());

            for (key, value) in map {
                // Allocate the key in the arena
                let key_ref = arena.alloc_str(key);

                // Convert the value
                let value_data = from_json(arena, value)?;

                // Add the pair to entries
                entries.push((key_ref, value_data));
            }

            // Allocate the entries in the arena
            let entries_slice = arena.alloc_slice_clone(&entries);
            Ok(DataValue::Object(entries_slice))
        }
    }
}

impl<'a> DataValue<'a> {
    /// Parse JSON string into DataValue
    ///
    /// Deserializes a JSON string into a DataValue using the provided arena allocator.
    ///
    /// # Arguments
    ///
    /// * `arena` - The arena allocator to store strings, arrays, and objects
    /// * `s` - The JSON string to parse
    ///
    /// # Returns
    ///
    /// Result containing the parsed DataValue or an error
    ///
    /// # Example
    ///
    /// ```
    /// # use datavalue_rs::{DataValue, Bump};
    /// let arena = Bump::new();
    /// let json = r#"{"name": "John", "age": 30}"#;
    ///
    /// let value = DataValue::from_str(&arena, json).unwrap();
    /// assert_eq!(value["name"].as_str(), Some("John"));
    /// ```
    pub fn from_str(arena: &'a Bump, s: &str) -> Result<Self> {
        from_str(arena, s)
    }

    /// Parse JSON from reader
    ///
    /// Reads JSON data from an io::Read source and parses it into a DataValue.
    ///
    /// # Arguments
    ///
    /// * `arena` - The arena allocator to store strings, arrays, and objects
    /// * `reader` - The reader to read JSON data from
    ///
    /// # Returns
    ///
    /// Result containing the parsed DataValue or an error
    ///
    /// # Example
    ///
    /// ```
    /// # use datavalue_rs::{DataValue, Bump};
    /// # use std::io::Cursor;
    /// let arena = Bump::new();
    ///
    /// // Create a reader from a string
    /// let json = r#"{"name": "John", "age": 30}"#;
    /// let reader = Cursor::new(json);
    ///
    /// let value = DataValue::from_reader(&arena, reader).unwrap();
    /// assert_eq!(value["name"].as_str(), Some("John"));
    /// ```
    pub fn from_reader<R: Read>(arena: &'a Bump, mut reader: R) -> Result<Self> {
        let mut buffer = String::new();
        reader.read_to_string(&mut buffer).map_err(Error::from)?;
        from_str(arena, &buffer)
    }

    /// Parse JSON from byte slice
    ///
    /// Parses a byte slice containing UTF-8 encoded JSON data into a DataValue.
    ///
    /// # Arguments
    ///
    /// * `arena` - The arena allocator to store strings, arrays, and objects
    /// * `v` - The byte slice containing JSON data
    ///
    /// # Returns
    ///
    /// Result containing the parsed DataValue or an error
    ///
    /// # Errors
    ///
    /// Returns an error if the byte slice is not valid UTF-8 or contains invalid JSON.
    ///
    /// # Example
    ///
    /// ```
    /// # use datavalue_rs::{DataValue, Bump};
    /// let arena = Bump::new();
    ///
    /// // JSON data as bytes
    /// let json_bytes = br#"{"name": "John", "age": 30}"#;
    ///
    /// let value = DataValue::from_slice(&arena, json_bytes).unwrap();
    /// assert_eq!(value["name"].as_str(), Some("John"));
    /// ```
    pub fn from_slice(arena: &'a Bump, v: &[u8]) -> Result<Self> {
        let s =
            std::str::from_utf8(v).map_err(|e| Error::syntax(format!("Invalid UTF-8: {}", e)))?;
        from_str(arena, s)
    }

    /// Convert from serde_json::Value
    ///
    /// Converts a serde_json::Value into a DataValue, allocating strings, arrays,
    /// and objects in the provided arena.
    ///
    /// # Arguments
    ///
    /// * `arena` - The arena allocator to store strings, arrays, and objects
    /// * `json` - The serde_json::Value to convert
    ///
    /// # Returns
    ///
    /// Result containing the converted DataValue or an error
    ///
    /// # Example
    ///
    /// ```
    /// # use datavalue_rs::{DataValue, Bump};
    /// # use serde_json::json;
    /// let arena = Bump::new();
    ///
    /// // Create a serde_json::Value
    /// let json_value = json!({
    ///     "name": "John",
    ///     "age": 30
    /// });
    ///
    /// let value = DataValue::from_json(&arena, &json_value).unwrap();
    /// assert_eq!(value["name"].as_str(), Some("John"));
    /// ```
    pub fn from_json(arena: &'a Bump, json: &serde_json::Value) -> Result<Self> {
        from_json(arena, json)
    }
}

// Implementation for serde Deserialize
impl<'de, 'a> serde::Deserialize<'de> for DataValue<'a>
where
    'de: 'a,
{
    /// Deserialize a DataValue from a serde Deserializer
    ///
    /// This implementation creates a leaked arena for DataValue allocation,
    /// which may cause memory leaks if used repeatedly. For most cases,
    /// prefer using from_str or from_json with an explicitly managed arena.
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        // First deserialize into a serde_json::Value
        let json = serde_json::Value::deserialize(deserializer)?;

        // Create a new arena for this deserialization
        // This isn't ideal as it causes a memory leak, but it's
        // needed because we can't store the arena reference
        let bump = Box::leak(Box::new(Bump::new()));

        // Convert to DataValue using the leaked arena
        from_json(bump, &json)
            .map_err(|e| serde::de::Error::custom(format!("Error converting to DataValue: {}", e)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_str_primitives() {
        let arena = Bump::new();

        // Test null
        let json = "null";
        let value = from_str(&arena, json).unwrap();
        assert!(matches!(value, DataValue::Null));

        // Test boolean
        let json = "true";
        let value = from_str(&arena, json).unwrap();
        assert!(matches!(value, DataValue::Bool(true)));

        // Test integer
        let json = "42";
        let value = from_str(&arena, json).unwrap();
        if let DataValue::Number(Number::Integer(i)) = value {
            assert_eq!(i, 42);
        } else {
            panic!("Expected integer");
        }

        // Test float
        let json = "3.14";
        let value = from_str(&arena, json).unwrap();
        if let DataValue::Number(Number::Float(f)) = value {
            assert!((f - 3.14).abs() < f64::EPSILON);
        } else {
            panic!("Expected float");
        }

        // Test string
        let json = r#""hello""#;
        let value = from_str(&arena, json).unwrap();
        if let DataValue::String(s) = value {
            assert_eq!(s, "hello");
        } else {
            panic!("Expected string");
        }
    }

    #[test]
    fn test_from_str_array() {
        let arena = Bump::new();

        let json = "[1, 2, 3]";
        let value = from_str(&arena, json).unwrap();

        if let DataValue::Array(arr) = value {
            assert_eq!(arr.len(), 3);

            // Check first element
            if let DataValue::Number(Number::Integer(i)) = arr[0] {
                assert_eq!(i, 1);
            } else {
                panic!("Expected integer at index 0");
            }
        } else {
            panic!("Expected array");
        }
    }

    #[test]
    fn test_from_str_object() {
        let arena = Bump::new();

        let json = r#"{"name": "John", "age": 30}"#;
        let value = from_str(&arena, json).unwrap();

        if let DataValue::Object(obj) = value {
            assert_eq!(obj.len(), 2);

            // Check for name key
            let name_entry = obj.iter().find(|(k, _)| *k == "name").unwrap();
            if let DataValue::String(s) = name_entry.1 {
                assert_eq!(s, "John");
            } else {
                panic!("Expected string for name");
            }

            // Check for age key
            let age_entry = obj.iter().find(|(k, _)| *k == "age").unwrap();
            if let DataValue::Number(Number::Integer(i)) = age_entry.1 {
                assert_eq!(i, 30);
            } else {
                panic!("Expected integer for age");
            }
        } else {
            panic!("Expected object");
        }
    }

    #[test]
    fn test_from_str_nested() {
        let arena = Bump::new();

        let json = r#"
        {
            "name": "John",
            "age": 30,
            "hobbies": ["reading", "coding"],
            "address": {
                "city": "New York",
                "country": "USA"
            }
        }
        "#;

        let value = from_str(&arena, json).unwrap();

        if let DataValue::Object(obj) = value {
            // Check address object
            let address_entry = obj.iter().find(|(k, _)| *k == "address").unwrap();
            if let DataValue::Object(addr) = address_entry.1 {
                // Check city field
                let city_entry = addr.iter().find(|(k, _)| *k == "city").unwrap();
                if let DataValue::String(s) = city_entry.1 {
                    assert_eq!(s, "New York");
                } else {
                    panic!("Expected string for city");
                }
            } else {
                panic!("Expected object for address");
            }

            // Check hobbies array
            let hobbies_entry = obj.iter().find(|(k, _)| *k == "hobbies").unwrap();
            if let DataValue::Array(hobbies) = hobbies_entry.1 {
                assert_eq!(hobbies.len(), 2);
                if let DataValue::String(s) = hobbies[0] {
                    assert_eq!(s, "reading");
                } else {
                    panic!("Expected string for first hobby");
                }
            } else {
                panic!("Expected array for hobbies");
            }
        } else {
            panic!("Expected object");
        }
    }

    #[test]
    fn test_from_json() {
        let arena = Bump::new();

        // Create a serde_json::Value
        let json_value = serde_json::json!({
            "name": "John",
            "age": 30,
            "hobbies": ["reading", "coding"]
        });

        // Convert to DataValue
        let value = from_json(&arena, &json_value).unwrap();

        // Verify the structure
        if let DataValue::Object(obj) = value {
            assert_eq!(obj.len(), 3);

            // Check name
            let name_entry = obj.iter().find(|(k, _)| *k == "name").unwrap();
            if let DataValue::String(s) = name_entry.1 {
                assert_eq!(s, "John");
            } else {
                panic!("Expected string for name");
            }

            // Check hobbies
            let hobbies_entry = obj.iter().find(|(k, _)| *k == "hobbies").unwrap();
            if let DataValue::Array(hobbies) = hobbies_entry.1 {
                assert_eq!(hobbies.len(), 2);
            } else {
                panic!("Expected array for hobbies");
            }
        } else {
            panic!("Expected object");
        }
    }
}
