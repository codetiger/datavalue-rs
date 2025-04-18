use datavalue_rs::{helpers, Bump, DataValue, Number};

#[test]
fn test_json_macro_basic_types() {
    let arena = Bump::new();

    // Test null
    let null_value = helpers::null();
    assert!(matches!(null_value, DataValue::Null));

    // Test booleans
    let true_value = helpers::boolean(true);
    assert!(matches!(true_value, DataValue::Bool(true)));

    let false_value = helpers::boolean(false);
    assert!(matches!(false_value, DataValue::Bool(false)));

    // Test string
    let string_value = helpers::string(&arena, "hello");
    if let DataValue::String(s) = string_value {
        assert_eq!(s, "hello");
    } else {
        panic!("Expected string");
    }
}

#[test]
fn test_json_macro_numbers() {
    // We don't need an arena for numeric values

    // Test integer
    let int_value = helpers::int(42);
    if let DataValue::Number(Number::Integer(i)) = int_value {
        assert_eq!(i, 42);
    } else {
        panic!("Expected integer number");
    }
}
