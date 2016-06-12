use params::{Map, Value};

pub fn validate_present(values: &Map, field: &str) -> Result<Option<Value>, String> {
    match values.find(&[field]) {
        Some(&Value::Null) |
        None => {
            Err(format!("The {} field must be present.",
                        field.to_lowercase().replace("_", " ")))
        }
        _ => Ok(None),
    }
}
