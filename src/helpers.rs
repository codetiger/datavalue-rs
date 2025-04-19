//! Utility functions for creating DataValue instances
//!
//! This module provides convenient functions for creating DataValue instances of
//! different types. These functions are similar to the constructor functions
//! in serde_json, but adapted for arena-based allocation where needed.
//!
//! The functions can be divided into two categories:
//! - Simple value constructors (`null()`, `boolean()`, `int()`, `float()`) that don't require an arena
//! - Complex value constructors (`string()`, `array()`, `object()`) that require an arena allocator

use crate::{
    datavalue::{DataValue, Number},
    Error, Result,
};
use bumpalo::Bump;
use chrono::{DateTime, Duration, Utc};

/// Creates a null DataValue
///
/// # Returns
///
/// A DataValue representing JSON null.
///
/// # Example
///
/// ```
/// # use datavalue_rs::helpers;
/// let null_value = helpers::null();
/// assert!(null_value.is_null());
/// ```
#[inline]
pub fn null() -> DataValue<'static> {
    DataValue::Null
}

/// Creates a boolean DataValue
///
/// # Arguments
///
/// * `value` - The boolean value to wrap
///
/// # Returns
///
/// A DataValue representing a JSON boolean value.
///
/// # Example
///
/// ```
/// # use datavalue_rs::helpers;
/// let true_value = helpers::boolean(true);
/// assert_eq!(true_value.as_bool(), Some(true));
///
/// let false_value = helpers::boolean(false);
/// assert_eq!(false_value.as_bool(), Some(false));
/// ```
#[inline]
pub fn boolean(value: bool) -> DataValue<'static> {
    DataValue::Bool(value)
}

/// Creates an integer DataValue
///
/// # Arguments
///
/// * `value` - The integer value to wrap
///
/// # Returns
///
/// A DataValue representing a JSON number with integer value.
///
/// # Example
///
/// ```
/// # use datavalue_rs::helpers;
/// let int_value = helpers::int(42);
/// assert_eq!(int_value.as_i64(), Some(42));
/// assert_eq!(int_value.as_f64(), Some(42.0)); // Also accessible as float
/// ```
#[inline]
pub fn int(value: i64) -> DataValue<'static> {
    DataValue::Number(Number::Integer(value))
}

/// Creates a float DataValue
///
/// # Arguments
///
/// * `value` - The floating point value to wrap
///
/// # Returns
///
/// A DataValue representing a JSON number with floating point value.
///
/// # Example
///
/// ```
/// # use datavalue_rs::helpers;
/// let float_value = helpers::float(3.14);
/// assert_eq!(float_value.as_f64(), Some(3.14));
/// assert_eq!(float_value.as_i64(), None); // Not accessible as integer
/// ```
#[inline]
pub fn float(value: f64) -> DataValue<'static> {
    DataValue::Number(Number::Float(value))
}

/// Creates a string DataValue
///
/// This function allocates the string in the provided arena and returns
/// a DataValue that references this string.
///
/// # Arguments
///
/// * `arena` - The arena allocator to store the string
/// * `value` - The string value to wrap
///
/// # Returns
///
/// A DataValue representing a JSON string.
///
/// # Example
///
/// ```
/// # use datavalue_rs::{helpers, Bump};
/// let arena = Bump::new();
/// let str_value = helpers::string(&arena, "hello world");
/// assert_eq!(str_value.as_str(), Some("hello world"));
/// ```
#[inline]
pub fn string<'a>(arena: &'a Bump, value: &str) -> DataValue<'a> {
    DataValue::String(arena.alloc_str(value))
}

/// Creates an array DataValue
///
/// This function allocates the array elements in the provided arena and returns
/// a DataValue that references these elements.
///
/// # Arguments
///
/// * `arena` - The arena allocator to store the array
/// * `values` - A vector of DataValue elements to include in the array
///
/// # Returns
///
/// A DataValue representing a JSON array.
///
/// # Example
///
/// ```
/// # use datavalue_rs::{helpers, Bump};
/// let arena = Bump::new();
///
/// // Create an array of primitive values
/// let arr = helpers::array(&arena, vec![
///     helpers::int(1),
///     helpers::int(2),
///     helpers::int(3),
/// ]);
///
/// assert!(arr.is_array());
/// let elements = arr.as_array().unwrap();
/// assert_eq!(elements.len(), 3);
/// assert_eq!(elements[0].as_i64(), Some(1));
///
/// // Create an array with mixed types
/// let mixed = helpers::array(&arena, vec![
///     helpers::boolean(true),
///     helpers::string(&arena, "hello"),
///     helpers::float(3.14),
/// ]);
///
/// assert_eq!(mixed.as_array().unwrap().len(), 3);
/// ```
#[inline]
pub fn array<'a>(arena: &'a Bump, values: Vec<DataValue<'a>>) -> DataValue<'a> {
    let elements_slice = arena.alloc_slice_clone(&values);
    DataValue::Array(elements_slice)
}

/// Creates an object DataValue
///
/// This function allocates the object entries in the provided arena and returns
/// a DataValue that references these entries.
///
/// # Arguments
///
/// * `arena` - The arena allocator to store the object
/// * `entries` - A vector of key-value pairs to include in the object
///
/// # Returns
///
/// A DataValue representing a JSON object.
///
/// # Note
///
/// The keys in the entries should already be allocated in the arena.
/// This is different from serde_json's behavior, where keys are owned strings.
///
/// # Example
///
/// ```
/// # use datavalue_rs::{helpers, Bump};
/// let arena = Bump::new();
///
/// // Create a simple object
/// let obj = helpers::object(&arena, vec![
///     (arena.alloc_str("name"), helpers::string(&arena, "John")),
///     (arena.alloc_str("age"), helpers::int(30)),
///     (arena.alloc_str("is_admin"), helpers::boolean(false)),
/// ]);
///
/// assert!(obj.is_object());
/// assert!(obj.contains_key("name"));
/// assert_eq!(obj["name"].as_str(), Some("John"));
/// assert_eq!(obj["age"].as_i64(), Some(30));
/// ```
#[inline]
pub fn object<'a>(arena: &'a Bump, entries: Vec<(&'a str, DataValue<'a>)>) -> DataValue<'a> {
    let entries_slice = arena.alloc_slice_clone(&entries);
    DataValue::Object(entries_slice)
}

/// Creates a datetime DataValue representing the current date and time
///
/// This function returns a DataValue representing the current date and time
/// in UTC timezone.
///
/// # Returns   
///
/// A DataValue representing a JSON datetime.
///
/// # Example
///
/// ```
/// # use datavalue_rs::helpers;
/// # use chrono::Utc;
/// let now = helpers::datetime_now();
/// assert!(now.as_datetime().is_some());
/// ```
#[inline]
pub fn datetime_now() -> DataValue<'static> {
    let dt = Utc::now();
    DataValue::DateTime(dt)
}

/// Creates a duration DataValue
///
/// This function returns a DataValue representing a duration in seconds.
///
/// # Returns
///
/// A DataValue representing a JSON duration.
///
/// # Example
///
/// ```
/// # use datavalue_rs::helpers;
/// # use chrono::Duration;
/// let duration_value = helpers::duration(10);
/// assert_eq!(duration_value.as_duration(), Some(Duration::seconds(10)));
/// ```
#[inline]
pub fn duration(value: i64) -> DataValue<'static> {
    let dur = Duration::seconds(value);
    DataValue::Duration(dur)
}

/// Creates a datetime DataValue from a string
///
/// This function parses a datetime string in RFC3339 format and returns a DataValue
/// representing the datetime.
///
/// # Arguments
///
/// * `value` - The datetime string to parse
///
/// # Returns
///
/// A Result containing a DataValue representing a JSON datetime, or an Error if parsing fails.
///
/// # Example
///
/// ```
/// # use datavalue_rs::helpers;
/// # use chrono::{DateTime, Utc};
/// let datetime_value = helpers::datetime("2021-01-01T00:00:00Z").unwrap();
/// assert!(datetime_value.as_datetime().is_some());
/// let dt: DateTime<Utc> = "2021-01-01T00:00:00Z".parse().unwrap();
/// assert_eq!(datetime_value.as_datetime(), Some(dt));
/// ```
#[inline]
pub fn datetime<'a>(value: &str) -> Result<DataValue<'a>> {
    DateTime::parse_from_rfc3339(value)
        .map(|dt| dt.with_timezone(&Utc))
        .or_else(|_| {
            // Try as ISO8601 without time
            chrono::NaiveDate::parse_from_str(value, "%Y-%m-%d")
                .map(|date| date.and_hms_opt(0, 0, 0).unwrap().and_utc())
        })
        .or_else(|_| {
            // Try other common formats (could add more as needed)
            chrono::NaiveDateTime::parse_from_str(value, "%Y-%m-%d %H:%M:%S").map(|dt| dt.and_utc())
        })
        .map_err(|e| Error::custom(e.to_string()))
        .map(DataValue::DateTime)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primitive_values() {
        // Test null
        assert!(matches!(null(), DataValue::Null));

        // Test boolean
        assert!(matches!(boolean(true), DataValue::Bool(true)));

        // Test integer
        match int(42) {
            DataValue::Number(Number::Integer(i)) => assert_eq!(i, 42),
            _ => panic!("Expected integer"),
        }

        // Test float
        match float(3.14) {
            DataValue::Number(Number::Float(f)) => assert!((f - 3.14).abs() < f64::EPSILON),
            _ => panic!("Expected float"),
        }
    }

    #[test]
    fn test_string() {
        let arena = Bump::new();
        match string(&arena, "hello") {
            DataValue::String(s) => {
                let s_str: &str = s;
                let expected: &str = "hello";
                assert!(s_str == expected);
            }
            _ => panic!("Expected string"),
        }
    }

    #[test]
    fn test_array() {
        let arena = Bump::new();
        let values = vec![int(1), int(2), int(3)];

        match array(&arena, values) {
            DataValue::Array(arr) => {
                assert_eq!(arr.len(), 3);
                match arr[0] {
                    DataValue::Number(Number::Integer(i)) => assert_eq!(i, 1),
                    _ => panic!("Expected integer"),
                }
            }
            _ => panic!("Expected array"),
        }
    }

    #[test]
    fn test_object() {
        let arena = Bump::new();

        // Create the entries with arena-allocated strings as keys
        let name_key = arena.alloc_str("name");
        let age_key = arena.alloc_str("age");

        // Create object entries with specific type
        let obj_entries: Vec<(&str, DataValue)> =
            vec![(name_key, string(&arena, "John")), (age_key, int(30))];

        // Pass entries to object function
        let obj_value = object(&arena, obj_entries);

        match obj_value {
            DataValue::Object(obj) => {
                assert_eq!(obj.len(), 2);

                // Basic validation without comparing strings directly
                let has_name = obj.iter().any(|(k, _)| *k == "name");
                let has_age = obj.iter().any(|(k, _)| *k == "age");

                assert!(has_name, "Object missing 'name' key");
                assert!(has_age, "Object missing 'age' key");
            }
            _ => panic!("Expected object"),
        }
    }
}
