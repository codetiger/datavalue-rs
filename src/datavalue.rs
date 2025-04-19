//! Core data types for representing JSON values
//!
//! This module defines the primary `DataValue` enum and related types,
//! which serve as an arena-based equivalent to `serde_json::Value`.

use chrono::{DateTime, Duration, Utc};
use std::fmt;
use std::ops::Index;

/// The primary data structure representing a JSON value.
/// A drop-in replacement for serde_json::Value with arena-based allocation for improved performance.
///
/// `DataValue` uses references to arena-allocated memory, which improves cache locality
/// and performance, but requires the user to manage the lifetime of the arena.
///
/// # Example
///
/// ```
/// use datavalue_rs::{DataValue, Bump};
///
/// let arena = Bump::new();
///
/// // Parse JSON string
/// let json_str = r#"{"name": "John", "age": 30}"#;
/// let value = DataValue::from_str(&arena, json_str).unwrap();
///
/// // Access values
/// assert_eq!(value["name"].as_str(), Some("John"));
/// assert_eq!(value["age"].as_i64(), Some(30));
/// ```
#[derive(Debug, Clone)]
pub enum DataValue<'a> {
    /// Represents a JSON null value.
    Null,
    /// Represents a JSON boolean value.
    Bool(bool),
    /// Represents a JSON number value (either integer or floating point).
    Number(Number),
    /// Represents a JSON string value, stored as a reference to a string in the arena.
    String(&'a str),
    /// Represents a JSON array, containing a list of DataValue elements.
    Array(&'a [DataValue<'a>]),
    /// Represents a JSON object, containing key-value pairs.
    Object(&'a [(&'a str, DataValue<'a>)]),
    /// Represents a JSON date-time value, stored as a reference to a string in the arena.
    DateTime(DateTime<Utc>),
    /// Represents a JSON duration value, stored as a reference to a string in the arena.
    Duration(Duration),
}

/// Represents the type of a DataValue
///
/// # Example
///
/// ```
/// # use datavalue_rs::{DataValue, DataValueType, helpers};
/// # use chrono::Utc;
///
/// // Check types of different values
/// assert_eq!(helpers::null().get_type(), DataValueType::Null);
/// assert_eq!(helpers::boolean(true).get_type(), DataValueType::Bool);
/// assert_eq!(helpers::int(42).get_type(), DataValueType::Integer);
/// assert_eq!(helpers::float(3.14).get_type(), DataValueType::Float);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataValueType {
    /// Null type
    Null,
    /// Boolean type
    Bool,
    /// Integer number type
    Integer,
    /// Float number type
    Float,
    /// String type
    String,
    /// Array type
    Array,
    /// Object type
    Object,
    /// DateTime type
    DateTime,
    /// Duration type
    Duration,
}

/// Represents a JSON number, either an integer or a floating point value.
///
/// This type allows for efficient storage of both integer and floating-point values
/// while preserving the original type information when possible.
///
/// # Example
///
/// ```
/// use datavalue_rs::{DataValue, Number};
///
/// // Integer value
/// let int_val = DataValue::Number(Number::Integer(42));
/// assert_eq!(int_val.as_i64(), Some(42));
/// assert_eq!(int_val.as_f64(), Some(42.0));
///
/// // Float value
/// let float_val = DataValue::Number(Number::Float(3.14));
/// assert_eq!(float_val.as_i64(), None);  // Can't convert to integer
/// assert_eq!(float_val.as_f64(), Some(3.14));
/// ```
#[derive(Debug, Clone, Copy)]
pub enum Number {
    /// Integer number representation
    Integer(i64),
    /// Floating point number representation
    Float(f64),
}

impl<'a> DataValue<'a> {
    /// Returns the type of this DataValue
    ///
    /// # Example
    ///
    /// ```
    /// # use datavalue_rs::{DataValue, DataValueType, Bump, helpers};
    /// # let arena = Bump::new();
    ///
    /// let null_val = helpers::null();
    /// assert_eq!(null_val.get_type(), DataValueType::Null);
    ///
    /// let str_val = helpers::string(&arena, "hello");
    /// assert_eq!(str_val.get_type(), DataValueType::String);
    ///
    /// let int_val = helpers::int(42);
    /// assert_eq!(int_val.get_type(), DataValueType::Integer);
    /// ```
    pub fn get_type(&self) -> DataValueType {
        match self {
            DataValue::Null => DataValueType::Null,
            DataValue::Bool(_) => DataValueType::Bool,
            DataValue::Number(Number::Integer(_)) => DataValueType::Integer,
            DataValue::Number(Number::Float(_)) => DataValueType::Float,
            DataValue::String(_) => DataValueType::String,
            DataValue::Array(_) => DataValueType::Array,
            DataValue::Object(_) => DataValueType::Object,
            DataValue::DateTime(_) => DataValueType::DateTime,
            DataValue::Duration(_) => DataValueType::Duration,
        }
    }

    /// Returns the boolean value if this DataValue is a boolean, otherwise None.
    ///
    /// # Example
    ///
    /// ```
    /// # use datavalue_rs::{DataValue, Bump};
    /// # let arena = Bump::new();
    /// // Creating a boolean value
    /// let bool_val = DataValue::Bool(true);
    /// assert_eq!(bool_val.as_bool(), Some(true));
    ///
    /// // Non-boolean value returns None
    /// let num_val = DataValue::Number(datavalue_rs::Number::Integer(42));
    /// assert_eq!(num_val.as_bool(), None);
    /// ```
    ///
    /// Equivalent to serde_json::Value::as_bool
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            DataValue::Bool(b) => Some(*b),
            _ => None,
        }
    }

    /// Returns the integer value if this DataValue is an integer number, otherwise None.
    ///
    /// # Example
    ///
    /// ```
    /// # use datavalue_rs::{DataValue, Number, Bump};
    /// # let arena = Bump::new();
    /// // Integer value
    /// let int_val = DataValue::Number(Number::Integer(42));
    /// assert_eq!(int_val.as_i64(), Some(42));
    ///
    /// // Float value returns None
    /// let float_val = DataValue::Number(Number::Float(3.14));
    /// assert_eq!(float_val.as_i64(), None);
    /// ```
    ///
    /// Equivalent to serde_json::Value::as_i64
    pub fn as_i64(&self) -> Option<i64> {
        match self {
            DataValue::Number(Number::Integer(i)) => Some(*i),
            _ => None,
        }
    }

    /// Returns the floating point value if this DataValue is a number, otherwise None.
    /// If the number is an integer, it will be converted to a floating point.
    ///
    /// # Example
    ///
    /// ```
    /// # use datavalue_rs::{DataValue, Number, Bump};
    /// # let arena = Bump::new();
    /// // Float value
    /// let float_val = DataValue::Number(Number::Float(3.14));
    /// assert_eq!(float_val.as_f64(), Some(3.14));
    ///
    /// // Integer value converted to float
    /// let int_val = DataValue::Number(Number::Integer(42));
    /// assert_eq!(int_val.as_f64(), Some(42.0));
    /// ```
    ///
    /// Equivalent to serde_json::Value::as_f64
    pub fn as_f64(&self) -> Option<f64> {
        match self {
            DataValue::Number(Number::Integer(i)) => Some(*i as f64),
            DataValue::Number(Number::Float(f)) => Some(*f),
            _ => None,
        }
    }

    /// Returns a reference to the string if this DataValue is a string, otherwise None.
    ///
    /// # Example
    ///
    /// ```
    /// # use datavalue_rs::{DataValue, Bump};
    /// # let arena = Bump::new();
    /// let s = arena.alloc_str("hello");
    /// let str_val = DataValue::String(s);
    /// assert_eq!(str_val.as_str(), Some("hello"));
    /// ```
    ///
    /// Equivalent to serde_json::Value::as_str
    pub fn as_str(&self) -> Option<&'a str> {
        match self {
            DataValue::String(s) => Some(s),
            _ => None,
        }
    }

    /// Returns a reference to the array if this DataValue is an array, otherwise None.
    ///
    /// # Example
    ///
    /// ```
    /// # use datavalue_rs::{DataValue, Bump, helpers};
    /// # let arena = Bump::new();
    /// let vals = vec![
    ///     DataValue::Number(datavalue_rs::Number::Integer(1)),
    ///     DataValue::Number(datavalue_rs::Number::Integer(2))
    /// ];
    /// let arr = arena.alloc_slice_clone(&vals);
    /// let arr_val = DataValue::Array(arr);
    ///
    /// let array_ref = arr_val.as_array().unwrap();
    /// assert_eq!(array_ref.len(), 2);
    /// ```
    ///
    /// Equivalent to serde_json::Value::as_array, but returns a slice instead of a Vec
    pub fn as_array(&self) -> Option<&[DataValue<'a>]> {
        match self {
            DataValue::Array(a) => Some(a),
            _ => None,
        }
    }

    /// Returns a reference to the object if this DataValue is an object, otherwise None.
    ///
    /// # Example
    ///
    /// ```
    /// # use datavalue_rs::{DataValue, Bump, helpers};
    /// # let arena = Bump::new();
    /// // Create an object using the helpers function (recommended approach)
    /// let obj = helpers::object(&arena, vec![
    ///     (arena.alloc_str("key"), helpers::boolean(true))
    /// ]);
    ///
    /// let obj_ref = obj.as_object().unwrap();
    /// assert_eq!(obj_ref.len(), 1);
    /// assert_eq!(obj_ref[0].0, "key");
    /// ```
    ///
    /// Equivalent to serde_json::Value::as_object, but returns a slice of key-value pairs
    /// instead of a Map
    pub fn as_object(&self) -> Option<&[(&'a str, DataValue<'a>)]> {
        match self {
            DataValue::Object(o) => Some(o),
            _ => None,
        }
    }

    /// Returns a reference to the date-time value if this DataValue is a date-time, otherwise None.
    ///
    /// # Example
    ///
    /// ```
    /// # use datavalue_rs::{DataValue};
    /// # use chrono::{DateTime, Utc};
    /// let dt = Utc::now();
    /// let dt_val = DataValue::DateTime(dt);
    /// assert!(dt_val.as_datetime().is_some());
    /// ```
    ///
    pub fn as_datetime(&self) -> Option<DateTime<Utc>> {
        match self {
            DataValue::DateTime(dt) => Some(*dt),
            _ => None,
        }
    }

    /// Returns a reference to the duration value if this DataValue is a duration, otherwise None.
    ///
    /// # Example
    ///
    /// ```
    /// # use datavalue_rs::{DataValue};
    /// # use chrono::Duration;
    /// let dur = Duration::seconds(10);
    /// let dur_val = DataValue::Duration(dur);
    /// assert_eq!(dur_val.as_duration(), Some(Duration::seconds(10)));
    /// ```
    ///
    pub fn as_duration(&self) -> Option<Duration> {
        match self {
            DataValue::Duration(dur) => Some(*dur),
            _ => None,
        }
    }

    /// Gets a reference to the DataValue associated with the given key if this DataValue is an object.
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
    /// let name = obj.get("name").unwrap();
    /// assert_eq!(name.as_str(), Some("John"));
    ///
    /// // Non-existent key returns None
    /// assert!(obj.get("address").is_none());
    /// ```
    ///
    /// Equivalent to serde_json::Value::get
    pub fn get(&self, key: &str) -> Option<&DataValue<'a>> {
        match self {
            DataValue::Object(o) => o.iter().find(|(k, _)| *k == key).map(|(_, v)| v),
            _ => None,
        }
    }

    /// Checks if this DataValue object contains the specified key.
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
    /// assert!(obj.contains_key("name"));
    /// assert!(!obj.contains_key("address"));
    /// ```
    ///
    /// This is similar to functionality provided by serde_json's Map object
    pub fn contains_key(&self, key: &str) -> bool {
        match self {
            DataValue::Object(o) => o.iter().any(|(k, _)| *k == key),
            _ => false,
        }
    }

    /// Gets a reference to the DataValue at the given index if this DataValue is an array.
    ///
    /// # Example
    ///
    /// ```
    /// # use datavalue_rs::{DataValue, Bump, helpers};
    /// # let arena = Bump::new();
    /// let arr = helpers::array(&arena, vec![
    ///     helpers::int(10),
    ///     helpers::int(20),
    ///     helpers::int(30),
    /// ]);
    ///
    /// let first = arr.get_index(0).unwrap();
    /// assert_eq!(first.as_i64(), Some(10));
    ///
    /// // Index out of bounds returns None
    /// assert!(arr.get_index(5).is_none());
    /// ```
    ///
    /// Equivalent to serde_json::Value::get for array indices
    pub fn get_index(&self, index: usize) -> Option<&DataValue<'a>> {
        match self {
            DataValue::Array(a) => a.get(index),
            _ => None,
        }
    }
}

// Implement Display trait instead of inherent to_string method
impl fmt::Display for DataValue<'_> {
    /// Formats the DataValue as a JSON string.
    ///
    /// This provides a compact JSON representation of the value without extra whitespace.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DataValue::Null => write!(f, "null"),
            DataValue::Bool(b) => write!(f, "{}", b),
            DataValue::Number(Number::Integer(i)) => write!(f, "{}", i),
            DataValue::Number(Number::Float(fl)) => write!(f, "{}", fl),
            DataValue::String(s) => write!(f, "\"{}\"", s.replace('\"', "\\\"")),
            DataValue::Array(arr) => {
                write!(f, "[")?;
                let mut first = true;
                for item in arr.iter() {
                    if !first {
                        write!(f, ",")?;
                    }
                    write!(f, "{}", item)?;
                    first = false;
                }
                write!(f, "]")
            }
            DataValue::Object(obj) => {
                write!(f, "{{")?;
                let mut first = true;
                for (key, value) in obj.iter() {
                    if !first {
                        write!(f, ",")?;
                    }
                    write!(f, "\"{}\":{}", key, value)?;
                    first = false;
                }
                write!(f, "}}")
            }
            DataValue::Duration(dur) => write!(f, "{}", dur),
            DataValue::DateTime(dt) => write!(f, "{}", dt),
        }
    }
}

impl<'a> Index<&str> for DataValue<'a> {
    type Output = DataValue<'a>;

    /// Accesses a DataValue by key, panicking if the key doesn't exist or value is not an object.
    ///
    /// # Panics
    ///
    /// Panics if the value is not an object or the key doesn't exist.
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
    /// let name = &obj["name"];
    /// assert_eq!(name.as_str(), Some("John"));
    /// ```
    fn index(&self, key: &str) -> &Self::Output {
        self.get(key)
            .unwrap_or_else(|| panic!("no entry found for key `{}`", key))
    }
}

impl<'a> Index<usize> for DataValue<'a> {
    type Output = DataValue<'a>;

    /// Accesses a DataValue by index, panicking if the index is out of bounds or value is not an array.
    ///
    /// # Panics
    ///
    /// Panics if the value is not an array or the index is out of bounds.
    ///
    /// # Example
    ///
    /// ```
    /// # use datavalue_rs::{DataValue, Bump, helpers};
    /// # let arena = Bump::new();
    /// let arr = helpers::array(&arena, vec![
    ///     helpers::int(10),
    ///     helpers::int(20),
    ///     helpers::int(30),
    /// ]);
    ///
    /// let second = &arr[1];
    /// assert_eq!(second.as_i64(), Some(20));
    /// ```
    fn index(&self, index: usize) -> &Self::Output {
        self.get_index(index)
            .unwrap_or_else(|| panic!("no element at index `{}`", index))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::helpers;
    use bumpalo::Bump;

    #[test]
    fn test_get_type() {
        // Test that get_type returns the correct type for each DataValue variant
        assert_eq!(DataValue::Null.get_type(), DataValueType::Null);
        assert_eq!(DataValue::Bool(true).get_type(), DataValueType::Bool);
        assert_eq!(
            DataValue::Number(Number::Integer(42)).get_type(),
            DataValueType::Integer
        );
        assert_eq!(
            DataValue::Number(Number::Float(3.14)).get_type(),
            DataValueType::Float
        );

        // For variants that require allocation, we'll use a Bump arena
        let arena = Bump::new();

        let string_val = DataValue::String(arena.alloc_str("hello"));
        assert_eq!(string_val.get_type(), DataValueType::String);

        // For array, use helpers which handle arena allocation correctly
        let array_val = helpers::array(&arena, vec![DataValue::Null]);
        assert_eq!(array_val.get_type(), DataValueType::Array);

        // For object, use helpers which handle arena allocation correctly
        let object_val = helpers::object(&arena, vec![(arena.alloc_str("key"), DataValue::Null)]);
        assert_eq!(object_val.get_type(), DataValueType::Object);

        let dt_val = DataValue::DateTime(Utc::now());
        assert_eq!(dt_val.get_type(), DataValueType::DateTime);

        let dur_val = DataValue::Duration(Duration::seconds(10));
        assert_eq!(dur_val.get_type(), DataValueType::Duration);
    }
}
