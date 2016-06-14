use params::{Map, Value};
use rustc_serialize::json::Json;

pub fn validate_json(values: &Map, field: &str) -> Result<Option<Value>, String> {
    match values.find(&[field]) {
        Some(&Value::String(ref value)) => {
            if value.is_empty() {
                // Allow empty values
                return Ok(None);
            }
            if Json::from_str(value).is_ok() {
                return Ok(None);
            }
            Err(format!("The {} field must contain a valid JSON string.",
                        field.to_lowercase().replace("_", " ")))
        }
        None => {
            // Allow empty values
            Ok(None)
        }
        _ => {
            Err(format!("The {} field must contain a valid JSON string.",
                        field.to_lowercase().replace("_", " ")))
        }
    }
}
