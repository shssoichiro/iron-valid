use params::{Map, Value};

pub fn validate_filled(values: &Map, field: &str) -> Result<Option<Value>, String> {
    match values.find(&[field]) {
        Some(&Value::String(ref value)) if value.is_empty() => {
            Err(format!("The {} field must be filled.",
                        field.to_lowercase().replace("_", " ")))
        }
        Some(&Value::Array(ref value)) if value.is_empty() => {
            Err(format!("The {} field must be filled.",
                        field.to_lowercase().replace("_", " ")))
        }
        Some(&Value::Map(ref value)) if value.is_empty() => {
            Err(format!("The {} field must be filled.",
                        field.to_lowercase().replace("_", " ")))
        }
        Some(&Value::Null) => {
            Err(format!("The {} field must be filled.",
                        field.to_lowercase().replace("_", " ")))
        }
        _ => Ok(None),
    }
}
