use params::{Map, Value};

pub fn validate_between(values: &Map,
                        field: &str,
                        min: isize,
                        max: isize)
                        -> Result<Option<Value>, String> {
    assert!(max >= min);

    match values.find(&[field]) {
        Some(&Value::String(ref value)) => {
            if value.is_empty() {
                // Allow empty values
                return Ok(None);
            }
            assert!(min >= 0);
            assert!(max >= 0);
            if value.len() >= min as usize && value.len() <= max as usize {
                Ok(None)
            } else {
                Err(format!("The {} field must be between {} and {} characters.",
                            field.to_lowercase().replace("_", " "),
                            min,
                            max))
            }
        }
        Some(&Value::U64(ref value)) => {
            if (min <= 0 || *value >= min as u64) && (max >= 0 && *value <= max as u64) {
                Ok(None)
            } else {
                Err(format!("The {} field must be between {} and {}.",
                            field.to_lowercase().replace("_", " "),
                            min,
                            max))
            }
        }
        Some(&Value::I64(ref value)) => {
            if *value >= min as i64 && *value <= max as i64 {
                Ok(None)
            } else {
                Err(format!("The {} field must be between {} and {}.",
                            field.to_lowercase().replace("_", " "),
                            min,
                            max))
            }
        }
        Some(&Value::F64(ref value)) => {
            if *value >= min as f64 && *value <= max as f64 {
                Ok(None)
            } else {
                Err(format!("The {} field must be between {} and {}.",
                            field.to_lowercase().replace("_", " "),
                            min,
                            max))
            }
        }
        Some(&Value::Array(ref value)) => {
            if value.is_empty() {
                // Allow empty values
                return Ok(None);
            }
            assert!(min >= 0);
            assert!(max >= 0);
            if value.len() >= min as usize && value.len() <= max as usize {
                Ok(None)
            } else {
                Err(format!("The {} field must have between {} and {} items.",
                            field.to_lowercase().replace("_", " "),
                            min,
                            max))
            }
        }
        Some(&Value::Map(ref value)) => {
            if value.is_empty() {
                // Allow empty values
                return Ok(None);
            }
            assert!(min >= 0);
            assert!(max >= 0);
            if value.len() >= min as usize && value.len() <= max as usize {
                Ok(None)
            } else {
                Err(format!("The {} field must have between {} and {} items.",
                            field.to_lowercase().replace("_", " "),
                            min,
                            max))
            }
        }
        Some(&Value::File(ref value)) => {
            assert!(min >= 0);
            assert!(max >= 0);
            if value.size() >> 10 >= min as u64 && value.size() >> 10 <= max as u64 {
                Ok(None)
            } else {
                Err(format!("The {} must be between {} and {} kilobytes.",
                            field.to_lowercase().replace("_", " "),
                            min,
                            max))
            }
        }
        None => {
            // Allow empty values
            Ok(None)
        }
        _ => {
            Err(format!("The {} field must have a size between {} and {}.",
                        field.to_lowercase().replace("_", " "),
                        min,
                        max))
        }
    }
}
