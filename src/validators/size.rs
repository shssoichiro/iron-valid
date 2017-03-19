use std::f64;

use params::{Map, Value};

pub fn validate_size(values: &Map, field: &[&str], target: isize) -> Result<Option<Value>, String> {
    match values.find(field) {
        Some(&Value::String(ref value)) => {
            if value.is_empty() {
                // Allow empty values
                return Ok(None);
            }
            assert!(target >= 0);
            if value.len() == target as usize {
                Ok(None)
            } else {
                Err(format!("The {} field must be {} characters.",
                            field.last()
                                .unwrap()
                                .to_lowercase()
                                .replace("_", " "),
                            target))
            }
        }
        Some(&Value::U64(ref value)) => {
            if *value == target as u64 {
                Ok(None)
            } else {
                Err(format!("The {} field must be {}.",
                            field.last()
                                .unwrap()
                                .to_lowercase()
                                .replace("_", " "),
                            target))
            }
        }
        Some(&Value::I64(ref value)) => {
            if *value == target as i64 {
                Ok(None)
            } else {
                Err(format!("The {} field must be {}.",
                            field.last()
                                .unwrap()
                                .to_lowercase()
                                .replace("_", " "),
                            target))
            }
        }
        Some(&Value::F64(ref value)) => {
            if (*value - target as f64).abs() < f64::EPSILON {
                Ok(None)
            } else {
                Err(format!("The {} field must be {}.",
                            field.last()
                                .unwrap()
                                .to_lowercase()
                                .replace("_", " "),
                            target))
            }
        }
        Some(&Value::Array(ref value)) => {
            if value.is_empty() {
                // Allow empty values
                return Ok(None);
            }
            assert!(target >= 0);
            if value.len() == target as usize {
                Ok(None)
            } else {
                Err(format!("The {} field must have {} items.",
                            field.last()
                                .unwrap()
                                .to_lowercase()
                                .replace("_", " "),
                            target))
            }
        }
        Some(&Value::Map(ref value)) => {
            if value.is_empty() {
                // Allow empty values
                return Ok(None);
            }
            assert!(target >= 0);
            if value.len() == target as usize {
                Ok(None)
            } else {
                Err(format!("The {} field must have {} items.",
                            field.last()
                                .unwrap()
                                .to_lowercase()
                                .replace("_", " "),
                            target))
            }
        }
        Some(&Value::File(ref value)) => {
            assert!(target >= 0);
            if value.size >> 10 == target as u64 {
                Ok(None)
            } else {
                Err(format!("The {} must be {} kilobytes.",
                            field.last()
                                .unwrap()
                                .to_lowercase()
                                .replace("_", " "),
                            target))
            }
        }
        None => {
            // Allow empty values
            Ok(None)
        }
        _ => {
            Err(format!("The {} field must have a size of {}.",
                        field.last()
                            .unwrap()
                            .to_lowercase()
                            .replace("_", " "),
                        target))
        }
    }
}
