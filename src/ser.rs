use crate::datavalue::{DataValue, Number};
use crate::error::{Error, Result};
use serde::ser::{Serialize, SerializeMap, SerializeSeq, Serializer};

// Standalone functions similar to serde_json
pub fn to_string(value: &DataValue<'_>) -> String {
    format!("{}", value)
}

pub fn to_string_pretty(value: &DataValue<'_>) -> String {
    // A simple pretty-printing implementation
    let mut result = String::new();
    to_string_pretty_internal(value, 0, &mut result);
    result
}

fn to_string_pretty_internal(value: &DataValue<'_>, indent: usize, output: &mut String) {
    let indent_str = "  ".repeat(indent);

    match value {
        DataValue::Null => output.push_str("null"),
        DataValue::Bool(b) => output.push_str(if *b { "true" } else { "false" }),
        DataValue::Number(Number::Integer(i)) => output.push_str(&i.to_string()),
        DataValue::Number(Number::Float(f)) => output.push_str(&f.to_string()),
        DataValue::String(s) => {
            output.push('"');
            output.push_str(&s.replace('\"', "\\\""));
            output.push('"');
        }
        DataValue::Array(arr) => {
            if arr.is_empty() {
                output.push_str("[]");
                return;
            }

            output.push_str("[\n");
            for (i, item) in arr.iter().enumerate() {
                output.push_str(&"  ".repeat(indent + 1));
                to_string_pretty_internal(item, indent + 1, output);
                if i < arr.len() - 1 {
                    output.push(',');
                }
                output.push('\n');
            }
            output.push_str(&indent_str);
            output.push(']');
        }
        DataValue::Object(obj) => {
            if obj.is_empty() {
                output.push_str("{}");
                return;
            }

            output.push_str("{\n");
            for (i, (key, value)) in obj.iter().enumerate() {
                output.push_str(&"  ".repeat(indent + 1));
                output.push('"');
                output.push_str(&key.replace('\"', "\\\""));
                output.push_str("\": ");
                to_string_pretty_internal(value, indent + 1, output);
                if i < obj.len() - 1 {
                    output.push(',');
                }
                output.push('\n');
            }
            output.push_str(&indent_str);
            output.push('}');
        }
    }
}

// Implement Serialize for DataValue to allow using serde's serialization
impl Serialize for DataValue<'_> {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            DataValue::Null => serializer.serialize_none(),
            DataValue::Bool(b) => serializer.serialize_bool(*b),
            DataValue::Number(Number::Integer(i)) => serializer.serialize_i64(*i),
            DataValue::Number(Number::Float(f)) => serializer.serialize_f64(*f),
            DataValue::String(s) => serializer.serialize_str(s),
            DataValue::Array(arr) => {
                let mut seq = serializer.serialize_seq(Some(arr.len()))?;
                for item in *arr {
                    seq.serialize_element(item)?;
                }
                seq.end()
            }
            DataValue::Object(obj) => {
                let mut map = serializer.serialize_map(Some(obj.len()))?;
                for (key, value) in *obj {
                    map.serialize_entry(key, value)?;
                }
                map.end()
            }
        }
    }
}

// Additional functions to write to writers
impl DataValue<'_> {
    /// Serialize to a writer
    pub fn to_writer<W: std::io::Write>(&self, mut writer: W) -> Result<()> {
        let s = format!("{}", self);
        writer.write_all(s.as_bytes()).map_err(Error::from)
    }

    /// Serialize to a writer with pretty-printing
    pub fn to_writer_pretty<W: std::io::Write>(&self, mut writer: W) -> Result<()> {
        let s = to_string_pretty(self);
        writer.write_all(s.as_bytes()).map_err(Error::from)
    }
}
