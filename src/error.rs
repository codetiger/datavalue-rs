use std::fmt;

/// Error type for DataValue operations
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
    pub fn syntax(msg: impl Into<String>) -> Self {
        Error::Syntax(msg.into())
    }

    /// Create a new expected type error
    pub fn expected_type(expected: impl Into<String>, found: impl Into<String>) -> Self {
        Error::ExpectedType {
            expected: expected.into(),
            found: found.into(),
        }
    }

    /// Create a new missing field error
    pub fn missing_field(field: impl Into<String>) -> Self {
        Error::MissingField(field.into())
    }

    /// Create a new out of bounds error
    pub fn out_of_bounds(index: usize) -> Self {
        Error::OutOfBounds(index)
    }

    /// Create a new custom error
    pub fn custom(msg: impl Into<String>) -> Self {
        Error::Custom(msg.into())
    }

    /// Create a new JSON error
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
pub type Result<T> = std::result::Result<T, Error>;
