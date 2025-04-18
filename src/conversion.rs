use crate::datavalue::{DataValue, Number};

impl From<i8> for DataValue<'_> {
    fn from(value: i8) -> Self {
        DataValue::Number(Number::Integer(value as i64))
    }
}

impl From<i16> for DataValue<'_> {
    fn from(value: i16) -> Self {
        DataValue::Number(Number::Integer(value as i64))
    }
}

impl From<i32> for DataValue<'_> {
    fn from(value: i32) -> Self {
        DataValue::Number(Number::Integer(value as i64))
    }
}

impl From<i64> for DataValue<'_> {
    fn from(value: i64) -> Self {
        DataValue::Number(Number::Integer(value))
    }
}

impl From<u8> for DataValue<'_> {
    fn from(value: u8) -> Self {
        DataValue::Number(Number::Integer(value as i64))
    }
}

impl From<u16> for DataValue<'_> {
    fn from(value: u16) -> Self {
        DataValue::Number(Number::Integer(value as i64))
    }
}

impl From<u32> for DataValue<'_> {
    fn from(value: u32) -> Self {
        DataValue::Number(Number::Integer(value as i64))
    }
}

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

impl From<f32> for DataValue<'_> {
    fn from(value: f32) -> Self {
        DataValue::Number(Number::Float(value as f64))
    }
}

impl From<f64> for DataValue<'_> {
    fn from(value: f64) -> Self {
        DataValue::Number(Number::Float(value))
    }
}

impl From<bool> for DataValue<'_> {
    fn from(value: bool) -> Self {
        DataValue::Bool(value)
    }
}

// Note: From<&str> or From<String> cannot be implemented here
// because DataValue requires arena-based allocation for strings
