use crate::datavalue::DataValue;

impl DataValue<'_> {
    /// Returns true if the value is null.
    /// Equivalent to serde_json::Value::is_null
    pub fn is_null(&self) -> bool {
        matches!(self, DataValue::Null)
    }

    /// Returns true if the value is a boolean.
    /// Equivalent to serde_json::Value::is_boolean
    pub fn is_bool(&self) -> bool {
        matches!(self, DataValue::Bool(_))
    }

    /// Returns true if the value is a number.
    /// Equivalent to serde_json::Value::is_number
    pub fn is_number(&self) -> bool {
        matches!(self, DataValue::Number(_))
    }

    /// Returns true if the value is a string.
    /// Equivalent to serde_json::Value::is_string
    pub fn is_string(&self) -> bool {
        matches!(self, DataValue::String(_))
    }

    /// Returns true if the value is an array.
    /// Equivalent to serde_json::Value::is_array
    pub fn is_array(&self) -> bool {
        matches!(self, DataValue::Array(_))
    }

    /// Returns true if the value is an object.
    /// Equivalent to serde_json::Value::is_object
    pub fn is_object(&self) -> bool {
        matches!(self, DataValue::Object(_))
    }

    /// Looks up a value by JSON pointer.
    /// Equivalent to serde_json::Value::pointer
    ///
    /// JSON Pointer defines a string syntax for identifying a specific value
    /// within a JSON document. A pointer is a sequence of "reference tokens"
    /// separated by `/`. Each reference token is a property name or array index.
    ///
    /// For example, given the JSON document:
    ///
    /// ```json
    /// {
    ///     "foo": ["bar", "baz"],
    ///     "": 0,
    ///     "a/b": 1,
    ///     "c%d": 2,
    ///     "e^f": 3,
    ///     "g|h": 4,
    ///     "i\\j": 5,
    ///     "k\"l": 6,
    ///     " ": 7,
    ///     "m~n": 8
    /// }
    /// ```
    ///
    /// The following JSON pointers evaluate to the accompanying values:
    ///
    /// ```text
    /// ""           // the whole document
    /// "/foo"       // ["bar", "baz"]
    /// "/foo/0"     // "bar"
    /// "/"          // 0
    /// "/a~1b"      // 1
    /// "/c%d"       // 2
    /// "/e^f"       // 3
    /// "/g|h"       // 4
    /// "/i\\j"      // 5
    /// "/k\"l"      // 6
    /// "/ "         // 7
    /// "/m~0n"      // 8
    /// ```
    ///
    /// Note that the JSON Pointer is not a query language. It can only refer to
    /// locations within the document defined by the structure of the document.
    pub fn pointer(&self, pointer: &str) -> Option<&Self> {
        // Empty pointer returns self
        if pointer.is_empty() {
            return Some(self);
        }

        // Pointer must start with '/'
        if !pointer.starts_with('/') {
            return None;
        }

        let mut current = self;
        for reference_token in pointer.split('/').skip(1) {
            let token = reference_token.replace("~1", "/").replace("~0", "~");

            current = match current {
                DataValue::Object(obj) => obj.iter().find(|(k, _)| k == &token).map(|(_, v)| v)?,
                DataValue::Array(arr) => {
                    if let Ok(index) = token.parse::<usize>() {
                        arr.get(index)?
                    } else {
                        return None;
                    }
                }
                _ => return None,
            };
        }

        Some(current)
    }

    // Note: The pointer_mut method is intentionally left as a no-op
    // because arena-based values make mutation difficult.
    // In serde_json::Value this method would return a mutable reference
    // but in our implementation we return None as a placeholder.
    /// Get a mutable reference to a value using a JSON pointer.
    /// Similar to serde_json::Value::pointer_mut, but always returns None
    /// due to limitations of arena-based allocation.
    pub fn pointer_mut(&mut self, _pointer: &str) -> Option<&mut Self> {
        // For arena-based DataValue, mutation is more complex due to lifetimes
        None
    }
}
