use std::fmt;
use std::ops::Index;

/// The primary data structure representing a JSON value.
/// A drop-in replacement for serde_json::Value with arena-based allocation for improved performance.
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
}

/// Represents a JSON number, either an integer or a floating point value.
#[derive(Debug, Clone, Copy)]
pub enum Number {
    /// Integer number representation
    Integer(i64),
    /// Floating point number representation
    Float(f64),
}

impl<'a> DataValue<'a> {
    /// Returns the boolean value if this DataValue is a boolean, otherwise None.
    /// Equivalent to serde_json::Value::as_bool
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            DataValue::Bool(b) => Some(*b),
            _ => None,
        }
    }

    /// Returns the integer value if this DataValue is an integer number, otherwise None.
    /// Equivalent to serde_json::Value::as_i64
    pub fn as_i64(&self) -> Option<i64> {
        match self {
            DataValue::Number(Number::Integer(i)) => Some(*i),
            _ => None,
        }
    }

    /// Returns the floating point value if this DataValue is a number, otherwise None.
    /// If the number is an integer, it will be converted to a floating point.
    /// Equivalent to serde_json::Value::as_f64
    pub fn as_f64(&self) -> Option<f64> {
        match self {
            DataValue::Number(Number::Integer(i)) => Some(*i as f64),
            DataValue::Number(Number::Float(f)) => Some(*f),
            _ => None,
        }
    }

    /// Returns a reference to the string if this DataValue is a string, otherwise None.
    /// Equivalent to serde_json::Value::as_str
    pub fn as_str(&self) -> Option<&'a str> {
        match self {
            DataValue::String(s) => Some(s),
            _ => None,
        }
    }

    /// Returns a reference to the array if this DataValue is an array, otherwise None.
    /// Equivalent to serde_json::Value::as_array, but returns a slice instead of a Vec
    pub fn as_array(&self) -> Option<&[DataValue<'a>]> {
        match self {
            DataValue::Array(a) => Some(a),
            _ => None,
        }
    }

    /// Returns a reference to the object if this DataValue is an object, otherwise None.
    /// Equivalent to serde_json::Value::as_object, but returns a slice of key-value pairs
    /// instead of a Map
    pub fn as_object(&self) -> Option<&[(&'a str, DataValue<'a>)]> {
        match self {
            DataValue::Object(o) => Some(o),
            _ => None,
        }
    }

    /// Gets a reference to the DataValue associated with the given key if this DataValue is an object.
    /// Equivalent to serde_json::Value::get
    pub fn get(&self, key: &str) -> Option<&DataValue<'a>> {
        match self {
            DataValue::Object(o) => o.iter().find(|(k, _)| *k == key).map(|(_, v)| v),
            _ => None,
        }
    }

    /// Checks if this DataValue object contains the specified key.
    /// This is similar to functionality provided by serde_json's Map object
    pub fn contains_key(&self, key: &str) -> bool {
        match self {
            DataValue::Object(o) => o.iter().any(|(k, _)| *k == key),
            _ => false,
        }
    }

    /// Gets a reference to the DataValue at the given index if this DataValue is an array.
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
        }
    }
}

impl<'a> Index<&str> for DataValue<'a> {
    type Output = DataValue<'a>;

    fn index(&self, key: &str) -> &Self::Output {
        self.get(key)
            .unwrap_or_else(|| panic!("no entry found for key `{}`", key))
    }
}

impl<'a> Index<usize> for DataValue<'a> {
    type Output = DataValue<'a>;

    fn index(&self, index: usize) -> &Self::Output {
        self.get_index(index)
            .unwrap_or_else(|| panic!("no element at index `{}`", index))
    }
}
