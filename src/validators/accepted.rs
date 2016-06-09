use params::{Map, Value};

pub fn validate_accepted(values: &Map, field: &str) -> Result<Option<Value>, String> {
    match values.find(&[field]) {
        Some(&Value::String(ref value)) => {
            let value = value.to_lowercase();
            if value == "yes" || value == "true" || value == "1" || value == "on" {
                Ok(Some(Value::Boolean(true)))
            } else {
                Err(format!("The {} must be accepted.",
                            field.to_lowercase().replace("_", " ")))
            }
        }
        Some(&Value::I64(ref value)) if *value == 1 => Ok(Some(Value::Boolean(true))),
        Some(&Value::U64(ref value)) if *value == 1 => Ok(Some(Value::Boolean(true))),
        Some(&Value::Boolean(ref value)) if *value => Ok(None),
        _ => {
            Err(format!("The {} must be accepted.",
                        field.to_lowercase().replace("_", " ")))
        }
    }
}
