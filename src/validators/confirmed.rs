use params::{Map, Value};
use std::f64;

pub fn validate_confirmed(values: &Map, field: &str) -> Result<Option<Value>, String> {
    match values.find(&[field]) {
        Some(&Value::String(ref value)) => {
            if value.is_empty() {
                return Ok(None);
            }
            match values.find(&[&[field, "_confirmation"].concat()]) {
                Some(&Value::String(ref value2)) if value == value2 => Ok(None),
                _ => {
                    Err(format!("The {} field must match the {} confirmation.",
                                field.to_lowercase().replace("_", " "),
                                field.to_lowercase().replace("_", " ")))
                }
            }
        }
        Some(&Value::U64(ref value)) => {
            match values.find(&[&[field, "_confirmation"].concat()]) {
                Some(&Value::U64(ref value2)) if value == value2 => Ok(None),
                _ => {
                    Err(format!("The {} field must match the {} confirmation.",
                                field.to_lowercase().replace("_", " "),
                                field.to_lowercase().replace("_", " ")))
                }
            }
        }
        Some(&Value::I64(ref value)) => {
            match values.find(&[&[field, "_confirmation"].concat()]) {
                Some(&Value::I64(ref value2)) if value == value2 => Ok(None),
                _ => {
                    Err(format!("The {} field must match the {} confirmation.",
                                field.to_lowercase().replace("_", " "),
                                field.to_lowercase().replace("_", " ")))
                }
            }
        }
        Some(&Value::F64(ref value)) => {
            match values.find(&[&[field, "_confirmation"].concat()]) {
                Some(&Value::F64(ref value2)) if (value - value2).abs() < f64::EPSILON => Ok(None),
                _ => {
                    Err(format!("The {} field must match the {} confirmation.",
                                field.to_lowercase().replace("_", " "),
                                field.to_lowercase().replace("_", " ")))
                }
            }
        }
        Some(&Value::Boolean(ref value)) => {
            match values.find(&[&[field, "_confirmation"].concat()]) {
                Some(&Value::Boolean(ref value2)) if value == value2 => Ok(None),
                _ => {
                    Err(format!("The {} field must match the {} confirmation.",
                                field.to_lowercase().replace("_", " "),
                                field.to_lowercase().replace("_", " ")))
                }
            }
        }
        Some(&Value::Array(ref value)) => {
            if value.is_empty() {
                return Ok(None);
            }
            match values.find(&[&[field, "_confirmation"].concat()]) {
                Some(&Value::Array(ref value2)) if value == value2 => Ok(None),
                _ => {
                    Err(format!("The {} field must match the {} confirmation.",
                                field.to_lowercase().replace("_", " "),
                                field.to_lowercase().replace("_", " ")))
                }
            }
        }
        Some(&Value::Map(ref value)) => {
            if value.is_empty() {
                return Ok(None);
            }
            match values.find(&[&[field, "_confirmation"].concat()]) {
                Some(&Value::Map(ref value2)) if value == value2 => Ok(None),
                _ => {
                    Err(format!("The {} field must match the {} confirmation.",
                                field.to_lowercase().replace("_", " "),
                                field.to_lowercase().replace("_", " ")))
                }
            }
        }
        None => {
            // Allow empty values
            Ok(None)
        }
        _ => {
            Err(format!("The {} field must match the {} confirmation.",
                        field.to_lowercase().replace("_", " "),
                        field.to_lowercase().replace("_", " ")))
        }
    }
}
