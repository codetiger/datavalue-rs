//! Error handling for DataValue operations
//!
//! This module defines the error types and result type used throughout the library.
//! It provides a comprehensive set of error variants to handle different failure cases
//! when working with JSON data.

use std::fmt;

/// Error type for DataValue operations
///
/// This type represents all possible errors that can occur when working with DataValue.
///
/// # Examples
///
/// ```
/// # use datavalue_rs::{Error, Result};
/// // Create a syntax error
/// let syntax_err = Error::syntax("Unexpected end of input");
/// assert!(syntax_err.to_string().contains("Syntax error"));
///
/// // Create a type error
/// let type_err = Error::expected_type("string", "number");
/// assert!(type_err.to_string().contains("Expected string, found number"));
/// ```
#[derive(Debug)]
pub enum Error {
    /// Syntax error during parsing
    Syntax(String),
    /// Expected a different type
    ExpectedType { expected: String, found: String },
    /// Missing a required field
    MissingField(String),
    /// Array index out of bounds
    OutOfBounds(usize),
    /// Custom error message
    Custom(String),
    /// IO error during serialization or deserialization
    Io(std::io::Error),
    /// JSON error from serde_json
    Json(String),
}

impl Error {
    /// Create a new syntax error
    ///
    /// # Arguments
    ///
    /// * `msg` - A message describing the syntax error
    ///
    /// # Example
    ///
    /// ```
    /// # use datavalue_rs::Error;
    /// let err = Error::syntax("Unexpected token '}' at line 2");
    /// ```
    pub fn syntax(msg: impl Into<String>) -> Self {
        Error::Syntax(msg.into())
    }

    /// Create a new expected type error
    ///
    /// # Arguments
    ///
    /// * `expected` - The type that was expected
    /// * `found` - The type that was found
    ///
    /// # Example
    ///
    /// ```
    /// # use datavalue_rs::Error;
    /// let err = Error::expected_type("string", "number");
    /// ```
    pub fn expected_type(expected: impl Into<String>, found: impl Into<String>) -> Self {
        Error::ExpectedType {
            expected: expected.into(),
            found: found.into(),
        }
    }

    /// Create a new missing field error
    ///
    /// # Arguments
    ///
    /// * `field` - The name of the missing field
    ///
    /// # Example
    ///
    /// ```
    /// # use datavalue_rs::Error;
    /// let err = Error::missing_field("name");
    /// ```
    pub fn missing_field(field: impl Into<String>) -> Self {
        Error::MissingField(field.into())
    }

    /// Create a new out of bounds error
    ///
    /// # Arguments
    ///
    /// * `index` - The out-of-bounds array index
    ///
    /// # Example
    ///
    /// ```
    /// # use datavalue_rs::Error;
    /// let err = Error::out_of_bounds(10);
    /// ```
    pub fn out_of_bounds(index: usize) -> Self {
        Error::OutOfBounds(index)
    }

    /// Create a new custom error
    ///
    /// # Arguments
    ///
    /// * `msg` - The custom error message
    ///
    /// # Example
    ///
    /// ```
    /// # use datavalue_rs::Error;
    /// let err = Error::custom("Something went wrong");
    /// ```
    pub fn custom(msg: impl Into<String>) -> Self {
        Error::Custom(msg.into())
    }

    /// Create a new JSON error
    ///
    /// # Arguments
    ///
    /// * `msg` - The JSON error message
    ///
    /// # Example
    ///
    /// ```
    /// # use datavalue_rs::Error;
    /// let err = Error::json("Invalid JSON format");
    /// ```
    pub fn json(msg: impl Into<String>) -> Self {
        Error::Json(msg.into())
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Syntax(msg) => write!(f, "Syntax error: {}", msg),
            Error::ExpectedType { expected, found } => {
                write!(f, "Expected {}, found {}", expected, found)
            }
            Error::MissingField(field) => write!(f, "Missing field: {}", field),
            Error::OutOfBounds(index) => write!(f, "Index out of bounds: {}", index),
            Error::Custom(msg) => write!(f, "{}", msg),
            Error::Io(err) => write!(f, "IO error: {}", err),
            Error::Json(msg) => write!(f, "JSON error: {}", msg),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::Io(err) => Some(err),
            _ => None,
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::Io(err)
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Error::Json(err.to_string())
    }
}

/// A specialized Result type for DataValue operations
///
/// This type is used throughout the library for functions that can fail.
///
/// # Example
///
/// ```
/// # use datavalue_rs::{DataValue, Error, Result, Bump, from_str};
///
/// fn process_json(json: &str) -> Result<String> {
///     let arena = Bump::new();
///     let value = from_str(&arena, json)?;
///     
///     if let Some(name) = value.get("user").and_then(|u| u.get("name")).and_then(|n| n.as_str()) {
///         Ok(format!("User: {}", name))
///     } else {
///         Err(Error::missing_field("name"))
///     }
/// }
///
/// // Test the function
/// let result = process_json(r#"{"user":{"name":"John"}}"#);
/// assert!(result.is_ok());
/// assert_eq!(result.unwrap(), "User: John");
///
/// let error_result = process_json(r#"{"user":{}}"#);
/// assert!(error_result.is_err());
/// ```
pub type Result<T> = std::result::Result<T, Error>;
