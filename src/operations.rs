//! Operations for DataValue
//!
//! This module provides operator overloading for DataValue instances.

use std::cmp::{Ordering, PartialEq, PartialOrd};
use std::ops::{Add, Div, Mul, Not, Sub};

use crate::{
    datavalue::{DataValue, Number},
    Error, Result,
};

// Implement operator traits directly on DataValue

impl Add for DataValue<'_> {
    type Output = Result<DataValue<'static>>;

    /// Implements the `+` operator for DataValue.
    ///
    /// # Behavior
    ///
    /// - Numbers are added mathematically
    /// - Operations that would require arena allocation will return an error
    ///
    /// # Arguments
    ///
    /// * `other` - The right-hand operand
    ///
    /// # Returns
    ///
    /// A Result containing the resulting DataValue, or an Error if the operation is invalid.
    fn add(self, other: Self) -> Self::Output {
        match (self, other) {
            // Integer + Integer
            (DataValue::Number(Number::Integer(a)), DataValue::Number(Number::Integer(b))) => {
                Ok(DataValue::Number(Number::Integer(a + b)))
            }
            // Integer + Float
            (DataValue::Number(Number::Integer(a)), DataValue::Number(Number::Float(b))) => {
                Ok(DataValue::Number(Number::Float(a as f64 + b)))
            }
            // Float + Integer
            (DataValue::Number(Number::Float(a)), DataValue::Number(Number::Integer(b))) => {
                Ok(DataValue::Number(Number::Float(a + b as f64)))
            }
            // Float + Float
            (DataValue::Number(Number::Float(a)), DataValue::Number(Number::Float(b))) => {
                Ok(DataValue::Number(Number::Float(a + b)))
            }
            // Invalid combinations
            (a, b) => Err(Error::custom(format!(
                "Cannot add values of types {:?} and {:?}",
                a.get_type(),
                b.get_type()
            ))),
        }
    }
}

impl Sub for DataValue<'_> {
    type Output = Result<DataValue<'static>>;

    /// Implements the `-` operator for DataValue.
    ///
    /// # Behavior
    ///
    /// - Numbers are subtracted mathematically
    /// - Other combinations result in an error
    ///
    /// # Arguments
    ///
    /// * `other` - The right-hand operand
    ///
    /// # Returns
    ///
    /// A Result containing the resulting DataValue, or an Error if the operation is invalid.
    fn sub(self, other: Self) -> Self::Output {
        match (self, other) {
            // Integer - Integer
            (DataValue::Number(Number::Integer(a)), DataValue::Number(Number::Integer(b))) => {
                Ok(DataValue::Number(Number::Integer(a - b)))
            }
            // Integer - Float
            (DataValue::Number(Number::Integer(a)), DataValue::Number(Number::Float(b))) => {
                Ok(DataValue::Number(Number::Float(a as f64 - b)))
            }
            // Float - Integer
            (DataValue::Number(Number::Float(a)), DataValue::Number(Number::Integer(b))) => {
                Ok(DataValue::Number(Number::Float(a - b as f64)))
            }
            // Float - Float
            (DataValue::Number(Number::Float(a)), DataValue::Number(Number::Float(b))) => {
                Ok(DataValue::Number(Number::Float(a - b)))
            }
            // Invalid combinations
            (a, b) => Err(Error::custom(format!(
                "Cannot subtract values of types {:?} and {:?}",
                a.get_type(),
                b.get_type()
            ))),
        }
    }
}

impl Mul for DataValue<'_> {
    type Output = Result<DataValue<'static>>;

    /// Implements the `*` operator for DataValue.
    ///
    /// # Behavior
    ///
    /// - Numbers are multiplied mathematically
    /// - Other combinations result in an error
    ///
    /// # Arguments
    ///
    /// * `other` - The right-hand operand
    ///
    /// # Returns
    ///
    /// A Result containing the resulting DataValue, or an Error if the operation is invalid.
    fn mul(self, other: Self) -> Self::Output {
        match (self, other) {
            // Integer * Integer
            (DataValue::Number(Number::Integer(a)), DataValue::Number(Number::Integer(b))) => {
                Ok(DataValue::Number(Number::Integer(a * b)))
            }
            // Integer * Float
            (DataValue::Number(Number::Integer(a)), DataValue::Number(Number::Float(b))) => {
                Ok(DataValue::Number(Number::Float(a as f64 * b)))
            }
            // Float * Integer
            (DataValue::Number(Number::Float(a)), DataValue::Number(Number::Integer(b))) => {
                Ok(DataValue::Number(Number::Float(a * b as f64)))
            }
            // Float * Float
            (DataValue::Number(Number::Float(a)), DataValue::Number(Number::Float(b))) => {
                Ok(DataValue::Number(Number::Float(a * b)))
            }
            // Invalid combinations
            (a, b) => Err(Error::custom(format!(
                "Cannot multiply values of types {:?} and {:?}",
                a.get_type(),
                b.get_type()
            ))),
        }
    }
}

impl Div for DataValue<'_> {
    type Output = Result<DataValue<'static>>;

    /// Implements the `/` operator for DataValue.
    ///
    /// # Behavior
    ///
    /// - Numbers are divided mathematically
    /// - Division by zero results in an error
    /// - Other combinations result in an error
    ///
    /// # Arguments
    ///
    /// * `other` - The right-hand operand
    ///
    /// # Returns
    ///
    /// A Result containing the resulting DataValue, or an Error if the operation is invalid.
    fn div(self, other: Self) -> Self::Output {
        match (self, other) {
            // Division by zero check for integers
            (_, DataValue::Number(Number::Integer(0))) => Err(Error::custom("Division by zero")),
            // Division by zero check for floats
            (_, DataValue::Number(Number::Float(0.0))) => Err(Error::custom("Division by zero")),
            // Integer / Integer
            (DataValue::Number(Number::Integer(a)), DataValue::Number(Number::Integer(b))) => {
                // If division is exact, keep as integer
                if a % b == 0 {
                    Ok(DataValue::Number(Number::Integer(a / b)))
                } else {
                    // Otherwise convert to float
                    Ok(DataValue::Number(Number::Float(a as f64 / b as f64)))
                }
            }
            // Integer / Float
            (DataValue::Number(Number::Integer(a)), DataValue::Number(Number::Float(b))) => {
                Ok(DataValue::Number(Number::Float(a as f64 / b)))
            }
            // Float / Integer
            (DataValue::Number(Number::Float(a)), DataValue::Number(Number::Integer(b))) => {
                Ok(DataValue::Number(Number::Float(a / b as f64)))
            }
            // Float / Float
            (DataValue::Number(Number::Float(a)), DataValue::Number(Number::Float(b))) => {
                Ok(DataValue::Number(Number::Float(a / b)))
            }
            // Invalid combinations
            (a, b) => Err(Error::custom(format!(
                "Cannot divide values of types {:?} and {:?}",
                a.get_type(),
                b.get_type()
            ))),
        }
    }
}

impl Not for DataValue<'_> {
    type Output = Result<DataValue<'static>>;

    /// Implements the `!` operator for DataValue.
    ///
    /// # Behavior
    ///
    /// - Boolean values are negated
    /// - Other types result in an error
    ///
    /// # Returns
    ///
    /// A Result containing the resulting DataValue, or an Error if the operation is invalid.
    fn not(self) -> Self::Output {
        match self {
            DataValue::Bool(a) => Ok(DataValue::Bool(!a)),
            a => Err(Error::custom(format!(
                "Cannot perform NOT on value of type {:?}",
                a.get_type()
            ))),
        }
    }
}

impl PartialEq for DataValue<'_> {
    /// Implements the `==` operator for DataValue.
    ///
    /// # Behavior
    ///
    /// Uses the `equals` function to determine equality.
    ///
    /// # Arguments
    ///
    /// * `other` - The right-hand operand
    ///
    /// # Returns
    ///
    /// True if the values are equal, false otherwise
    fn eq(&self, other: &Self) -> bool {
        equals(self, other)
    }
}

impl PartialOrd for DataValue<'_> {
    /// Implements the comparison operators for DataValue.
    ///
    /// # Behavior
    ///
    /// - Numbers are compared by value
    /// - Strings are compared lexicographically
    /// - Other types or mixed types return None
    ///
    /// # Arguments
    ///
    /// * `other` - The right-hand operand
    ///
    /// # Returns
    ///
    /// Some(Ordering) if the comparison is valid, None otherwise
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (less_than(self, other), equals(self, other)) {
            (Ok(true), _) => Some(Ordering::Less),
            (_, true) => Some(Ordering::Equal),
            (Ok(false), false) => Some(Ordering::Greater),
            _ => None,
        }
    }
}

// Private helper functions

fn equals(left: &DataValue, right: &DataValue) -> bool {
    match (left, right) {
        // Null == Null
        (DataValue::Null, DataValue::Null) => true,

        // Bool == Bool
        (DataValue::Bool(a), DataValue::Bool(b)) => a == b,

        // Number == Number (allowing integer/float comparison)
        (DataValue::Number(Number::Integer(a)), DataValue::Number(Number::Integer(b))) => a == b,
        (DataValue::Number(Number::Float(a)), DataValue::Number(Number::Float(b))) => a == b,
        (DataValue::Number(Number::Integer(a)), DataValue::Number(Number::Float(b))) => {
            *a as f64 == *b
        }
        (DataValue::Number(Number::Float(a)), DataValue::Number(Number::Integer(b))) => {
            *a == *b as f64
        }

        // String == String
        (DataValue::String(a), DataValue::String(b)) => a == b,

        // Array == Array
        (DataValue::Array(a), DataValue::Array(b)) => {
            if a.len() != b.len() {
                return false;
            }
            a.iter()
                .zip(b.iter())
                .all(|(a_elem, b_elem)| equals(a_elem, b_elem))
        }

        // Object == Object
        (DataValue::Object(a), DataValue::Object(b)) => {
            if a.len() != b.len() {
                return false;
            }

            // For each key in a, find matching key in b and compare values
            a.iter().all(|(a_key, a_val)| {
                b.iter()
                    .find(|(b_key, _)| a_key == b_key)
                    .is_some_and(|(_, b_val)| equals(a_val, b_val))
            })
        }

        // DateTime == DateTime
        (DataValue::DateTime(a), DataValue::DateTime(b)) => a == b,

        // Duration == Duration
        (DataValue::Duration(a), DataValue::Duration(b)) => a == b,

        // Different types are never equal
        _ => false,
    }
}

fn less_than(left: &DataValue, right: &DataValue) -> Result<bool> {
    match (left, right) {
        // Number < Number
        (DataValue::Number(Number::Integer(a)), DataValue::Number(Number::Integer(b))) => Ok(a < b),
        (DataValue::Number(Number::Float(a)), DataValue::Number(Number::Float(b))) => Ok(a < b),
        (DataValue::Number(Number::Integer(a)), DataValue::Number(Number::Float(b))) => {
            Ok((*a as f64) < *b)
        }
        (DataValue::Number(Number::Float(a)), DataValue::Number(Number::Integer(b))) => {
            Ok(*a < (*b as f64))
        }

        // String < String
        (DataValue::String(a), DataValue::String(b)) => Ok(a < b),

        // Invalid combinations
        (a, b) => Err(Error::custom(format!(
            "Cannot compare values of types {:?} and {:?} with <",
            a.get_type(),
            b.get_type()
        ))),
    }
}

#[cfg(test)]
mod tests {
    use crate::helpers;

    #[test]
    fn test_operator_add() {
        // Test integer addition
        let a = helpers::int(5);
        let b = helpers::int(3);
        let result = (a + b).unwrap();
        assert_eq!(result.as_i64(), Some(8));

        // Test mixed addition
        let a = helpers::int(5);
        let b = helpers::float(2.5);
        let result = (a + b).unwrap();
        assert_eq!(result.as_f64(), Some(7.5));
    }

    #[test]
    fn test_operator_subtract() {
        let a = helpers::int(10);
        let b = helpers::int(4);
        let result = (a - b).unwrap();
        assert_eq!(result.as_i64(), Some(6));
    }

    #[test]
    fn test_operator_multiply() {
        // Test number multiplication
        let a = helpers::int(5);
        let b = helpers::int(3);
        let result = (a * b).unwrap();
        assert_eq!(result.as_i64(), Some(15));
    }

    #[test]
    fn test_operator_equals() {
        let a = helpers::int(5);
        let b = helpers::int(5);
        let c = helpers::int(10);

        assert!(a == b);
        assert!(a != c);
    }

    #[test]
    fn test_operator_compare() {
        let a = helpers::int(5);
        let b = helpers::int(10);
        let c = helpers::int(5);

        assert!(a < b);
        assert!(b > a);
        assert!(a <= c);
        assert!(a >= c);
    }
}
