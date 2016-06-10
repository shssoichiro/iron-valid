use params::{Map, Value};

pub fn validate_alpha(values: &Map, field: &str) -> Result<Option<Value>, String> {
    match values.find(&[field]) {
        Some(&Value::String(ref value)) => {
            if value.is_empty() {
                // Allow empty values
                return Ok(None);
            }
            if value.chars().filter(|c| !c.is_alphabetic()).count() == 0 {
                return Ok(None);
            }
            Err(format!("The {} field may only contain alphabetic characters.",
                        field.to_lowercase().replace("_", " ")))
        }
        None => {
            // Allow empty values
            Ok(None)
        }
        _ => {
            Err(format!("The {} field may only contain alphabetic characters.",
                        field.to_lowercase().replace("_", " ")))
        }
    }
}
