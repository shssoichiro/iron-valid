use params::{Map, Value};

pub fn validate_integer(values: &Map, field: &str) -> Result<Option<Value>, String> {
    match values.find(&[field]) {
        Some(&Value::String(ref value)) => {
            if value.is_empty() {
                // Allow empty values
                return Ok(None);
            }
            let positive = value.parse::<u64>();
            match positive {
                Ok(uvalue) => Ok(Some(Value::U64(uvalue))),
                Err(_) => {
                    let negative = value.parse::<i64>();
                    match negative {
                        Ok(ivalue) => Ok(Some(Value::I64(ivalue))),
                        Err(_) => {
                            Err(format!("The {} field must be an integer.",
                                        field.to_lowercase().replace("_", " ")))
                        }
                    }
                }
            }
        }
        Some(&Value::U64(_)) |
        Some(&Value::I64(_)) => Ok(None),
        None => {
            // Allow empty values
            Ok(None)
        }
        _ => {
            Err(format!("The {} field must be an integer.",
                        field.to_lowercase().replace("_", " ")))
        }
    }
}
