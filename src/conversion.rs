//! Conversion traits for DataValue
//!
//! This module provides implementations of the `From` trait for various Rust primitive types,
//! allowing easy conversion to `DataValue`. Note that string conversions require arena allocation
//! and thus can't be implemented directly with the `From` trait.

use crate::datavalue::{DataValue, Number};

/// Create DataValue from i8
///
/// Converts to a Number::Integer variant.
///
/// # Example
/// ```
/// # use datavalue_rs::DataValue;
/// let value: DataValue = 42i8.into();
/// assert_eq!(value.as_i64(), Some(42));
/// ```
impl From<i8> for DataValue<'_> {
    fn from(value: i8) -> Self {
        DataValue::Number(Number::Integer(value as i64))
    }
}

/// Create DataValue from i16
///
/// Converts to a Number::Integer variant.
impl From<i16> for DataValue<'_> {
    fn from(value: i16) -> Self {
        DataValue::Number(Number::Integer(value as i64))
    }
}

/// Create DataValue from i32
///
/// Converts to a Number::Integer variant.
impl From<i32> for DataValue<'_> {
    fn from(value: i32) -> Self {
        DataValue::Number(Number::Integer(value as i64))
    }
}

/// Create DataValue from i64
///
/// Converts to a Number::Integer variant.
impl From<i64> for DataValue<'_> {
    fn from(value: i64) -> Self {
        DataValue::Number(Number::Integer(value))
    }
}

/// Create DataValue from u8
///
/// Converts to a Number::Integer variant.
impl From<u8> for DataValue<'_> {
    fn from(value: u8) -> Self {
        DataValue::Number(Number::Integer(value as i64))
    }
}

/// Create DataValue from u16
///
/// Converts to a Number::Integer variant.
impl From<u16> for DataValue<'_> {
    fn from(value: u16) -> Self {
        DataValue::Number(Number::Integer(value as i64))
    }
}

/// Create DataValue from u32
///
/// Converts to a Number::Integer variant.
impl From<u32> for DataValue<'_> {
    fn from(value: u32) -> Self {
        DataValue::Number(Number::Integer(value as i64))
    }
}

/// Create DataValue from u64
///
/// Converts to a Number::Integer variant for values that fit in i64.
/// For larger values, converts to Number::Float to avoid overflow.
impl From<u64> for DataValue<'_> {
    fn from(value: u64) -> Self {
        // Handle potential overflow for u64 values larger than i64::MAX
        if value <= i64::MAX as u64 {
            DataValue::Number(Number::Integer(value as i64))
        } else {
            DataValue::Number(Number::Float(value as f64))
        }
    }
}

/// Create DataValue from usize
///
/// Converts to a Number::Integer variant for values that fit in i64.
/// For larger values, converts to Number::Float to avoid overflow.
impl From<usize> for DataValue<'_> {
    fn from(value: usize) -> Self {
        // Handle potential overflow for usize values larger than i64::MAX
        if value <= i64::MAX as usize {
            DataValue::Number(Number::Integer(value as i64))
        } else {
            DataValue::Number(Number::Float(value as f64))
        }
    }
}

/// Create DataValue from f32
///
/// Converts to a Number::Float variant.
impl From<f32> for DataValue<'_> {
    fn from(value: f32) -> Self {
        DataValue::Number(Number::Float(value as f64))
    }
}

/// Create DataValue from f64
///
/// Converts to a Number::Float variant.
impl From<f64> for DataValue<'_> {
    fn from(value: f64) -> Self {
        DataValue::Number(Number::Float(value))
    }
}

/// Create DataValue from bool
///
/// Converts to a Bool variant.
impl From<bool> for DataValue<'_> {
    fn from(value: bool) -> Self {
        DataValue::Bool(value)
    }
}

// Note: From<&str> or From<String> cannot be implemented here
// because DataValue requires arena-based allocation for strings
