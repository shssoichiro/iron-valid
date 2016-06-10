use params::{Map, Value};

pub fn validate_array(values: &Map, field: &str) -> Result<Option<Value>, String> {
    match values.find(&[field]) {
        Some(&Value::Array(_)) => Ok(None),
        None => {
            // Allow empty values
            Ok(None)
        }
        _ => {
            Err(format!("The {} field must be an array.",
                        field.to_lowercase().replace("_", " ")))
        }
    }
}
