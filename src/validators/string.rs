use params::{Map, Value};

pub fn validate_string(values: &Map, field: &[&str]) -> Result<Option<Value>, String> {
    match values.find(field) {
        Some(&Value::String(_)) => Ok(None),
        None => {
            // Allow empty values
            Ok(None)
        }
        _ => {
            Err(format!("The {} field must be a string.",
                        field.last()
                            .unwrap()
                            .to_lowercase()
                            .replace("_", " ")))
        }
    }
}
