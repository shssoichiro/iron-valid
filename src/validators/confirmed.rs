use std::f64;

use params::{Map, Value};

pub fn validate_confirmed(values: &Map, field: &[&str]) -> Result<Option<Value>, String> {
    let last_confirmation = [field.last().unwrap(), "_confirmation"].concat();
    let mut confirmation_field = field.to_owned();
    *confirmation_field.last_mut().unwrap() = &last_confirmation;
    match values.find(field) {
        Some(&Value::String(ref value)) => {
            if value.is_empty() {
                return Ok(None);
            }
            match values.find(&confirmation_field) {
                Some(&Value::String(ref value2)) if value == value2 => Ok(None),
                _ => {
                    Err(format!("The {} field must match the {} confirmation.",
                                field.last()
                                    .unwrap()
                                    .to_lowercase()
                                    .replace("_", " "),
                                field.last()
                                    .unwrap()
                                    .to_lowercase()
                                    .replace("_", " ")))
                }
            }
        }
        Some(&Value::U64(ref value)) => {
            match values.find(&confirmation_field) {
                Some(&Value::U64(ref value2)) if value == value2 => Ok(None),
                _ => {
                    Err(format!("The {} field must match the {} confirmation.",
                                field.last()
                                    .unwrap()
                                    .to_lowercase()
                                    .replace("_", " "),
                                field.last()
                                    .unwrap()
                                    .to_lowercase()
                                    .replace("_", " ")))
                }
            }
        }
        Some(&Value::I64(ref value)) => {
            match values.find(&confirmation_field) {
                Some(&Value::I64(ref value2)) if value == value2 => Ok(None),
                _ => {
                    Err(format!("The {} field must match the {} confirmation.",
                                field.last()
                                    .unwrap()
                                    .to_lowercase()
                                    .replace("_", " "),
                                field.last()
                                    .unwrap()
                                    .to_lowercase()
                                    .replace("_", " ")))
                }
            }
        }
        Some(&Value::F64(ref value)) => {
            match values.find(&confirmation_field) {
                Some(&Value::F64(ref value2)) if (value - value2).abs() < f64::EPSILON => Ok(None),
                _ => {
                    Err(format!("The {} field must match the {} confirmation.",
                                field.last()
                                    .unwrap()
                                    .to_lowercase()
                                    .replace("_", " "),
                                field.last()
                                    .unwrap()
                                    .to_lowercase()
                                    .replace("_", " ")))
                }
            }
        }
        Some(&Value::Boolean(ref value)) => {
            match values.find(&confirmation_field) {
                Some(&Value::Boolean(ref value2)) if value == value2 => Ok(None),
                _ => {
                    Err(format!("The {} field must match the {} confirmation.",
                                field.last()
                                    .unwrap()
                                    .to_lowercase()
                                    .replace("_", " "),
                                field.last()
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
            match values.find(&confirmation_field) {
                Some(&Value::Array(ref value2)) if value == value2 => Ok(None),
                _ => {
                    Err(format!("The {} field must match the {} confirmation.",
                                field.last()
                                    .unwrap()
                                    .to_lowercase()
                                    .replace("_", " "),
                                field.last()
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
            match values.find(&confirmation_field) {
                Some(&Value::Map(ref value2)) if value == value2 => Ok(None),
                _ => {
                    Err(format!("The {} field must match the {} confirmation.",
                                field.last()
                                    .unwrap()
                                    .to_lowercase()
                                    .replace("_", " "),
                                field.last()
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
            Err(format!("The {} field must match the {} confirmation.",
                        field.last()
                            .unwrap()
                            .to_lowercase()
                            .replace("_", " "),
                        field.last()
                            .unwrap()
                            .to_lowercase()
                            .replace("_", " ")))
        }
    }
}
