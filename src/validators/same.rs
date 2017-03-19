use std::f64;

use params::{Map, Value};

pub fn validate_same(values: &Map,
                     field: &[&str],
                     other: &[&str])
                     -> Result<Option<Value>, String> {
    match values.find(field) {
        Some(&Value::String(ref value)) => {
            if value.is_empty() {
                return Ok(None);
            }
            match values.find(other) {
                Some(&Value::String(ref value2)) if value == value2 => Ok(None),
                _ => {
                    Err(format!("The {} field must match the {} field.",
                                field.last()
                                    .unwrap()
                                    .to_lowercase()
                                    .replace("_", " "),
                                other.last()
                                    .unwrap()
                                    .to_lowercase()
                                    .replace("_", " ")))
                }
            }
        }
        Some(&Value::U64(ref value)) => {
            match values.find(other) {
                Some(&Value::U64(ref value2)) if value == value2 => Ok(None),
                _ => {
                    Err(format!("The {} field must match the {} field.",
                                field.last()
                                    .unwrap()
                                    .to_lowercase()
                                    .replace("_", " "),
                                other.last()
                                    .unwrap()
                                    .to_lowercase()
                                    .replace("_", " ")))
                }
            }
        }
        Some(&Value::I64(ref value)) => {
            match values.find(other) {
                Some(&Value::I64(ref value2)) if value == value2 => Ok(None),
                _ => {
                    Err(format!("The {} field must match the {} field.",
                                field.last()
                                    .unwrap()
                                    .to_lowercase()
                                    .replace("_", " "),
                                other.last()
                                    .unwrap()
                                    .to_lowercase()
                                    .replace("_", " ")))
                }
            }
        }
        Some(&Value::F64(ref value)) => {
            match values.find(other) {
                Some(&Value::F64(ref value2)) if (value - value2).abs() < f64::EPSILON => Ok(None),
                _ => {
                    Err(format!("The {} field must match the {} field.",
                                field.last()
                                    .unwrap()
                                    .to_lowercase()
                                    .replace("_", " "),
                                other.last()
                                    .unwrap()
                                    .to_lowercase()
                                    .replace("_", " ")))
                }
            }
        }
        Some(&Value::Boolean(ref value)) => {
            match values.find(other) {
                Some(&Value::Boolean(ref value2)) if value == value2 => Ok(None),
                _ => {
                    Err(format!("The {} field must match the {} field.",
                                field.last()
                                    .unwrap()
                                    .to_lowercase()
                                    .replace("_", " "),
                                other.last()
                                    .unwrap()
                                    .to_lowercase()
                                    .replace("_", " ")))
                }
            }
        }
        Some(&Value::Array(ref value)) => {
            if value.is_empty() {
                return Ok(None);
            }
            match values.find(other) {
                Some(&Value::Array(ref value2)) if value == value2 => Ok(None),
                _ => {
                    Err(format!("The {} field must match the {} field.",
                                field.last()
                                    .unwrap()
                                    .to_lowercase()
                                    .replace("_", " "),
                                other.last()
                                    .unwrap()
                                    .to_lowercase()
                                    .replace("_", " ")))
                }
            }
        }
        Some(&Value::Map(ref value)) => {
            if value.is_empty() {
                return Ok(None);
            }
            match values.find(other) {
                Some(&Value::Map(ref value2)) if value == value2 => Ok(None),
                _ => {
                    Err(format!("The {} field must match the {} field.",
                                field.last()
                                    .unwrap()
                                    .to_lowercase()
                                    .replace("_", " "),
                                other.last()
                                    .unwrap()
                                    .to_lowercase()
                                    .replace("_", " ")))
                }
            }
        }
        None => {
            // Allow empty values
            Ok(None)
        }
        _ => {
            Err(format!("The {} field must match the {} field.",
                        field.last()
                            .unwrap()
                            .to_lowercase()
                            .replace("_", " "),
                        other.last()
                            .unwrap()
                            .to_lowercase()
                            .replace("_", " ")))
        }
    }
}
