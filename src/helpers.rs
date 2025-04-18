//! Utility functions for creating DataValue instances

use crate::datavalue::{DataValue, Number};
use bumpalo::Bump;

/// Creates a null DataValue
#[inline]
pub fn null() -> DataValue<'static> {
    DataValue::Null
}

/// Creates a boolean DataValue
#[inline]
pub fn boolean(value: bool) -> DataValue<'static> {
    DataValue::Bool(value)
}

/// Creates an integer DataValue
#[inline]
pub fn int(value: i64) -> DataValue<'static> {
    DataValue::Number(Number::Integer(value))
}

/// Creates a float DataValue
#[inline]
pub fn float(value: f64) -> DataValue<'static> {
    DataValue::Number(Number::Float(value))
}

/// Creates a string DataValue (requires an arena)
#[inline]
pub fn string<'a>(arena: &'a Bump, value: &str) -> DataValue<'a> {
    DataValue::String(arena.alloc_str(value))
}

/// Creates an array DataValue (requires an arena)
#[inline]
pub fn array<'a>(arena: &'a Bump, values: Vec<DataValue<'a>>) -> DataValue<'a> {
    let elements_slice = arena.alloc_slice_clone(&values);
    DataValue::Array(elements_slice)
}

/// Creates an object DataValue (requires an arena)
#[inline]
pub fn object<'a>(arena: &'a Bump, entries: Vec<(&'a str, DataValue<'a>)>) -> DataValue<'a> {
    let entries_slice = arena.alloc_slice_clone(&entries);
    DataValue::Object(entries_slice)
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
