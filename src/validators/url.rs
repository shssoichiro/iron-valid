use params::{Map, Value};
use url::Url;

pub fn validate_url(values: &Map, field: &[&str]) -> Result<Option<Value>, String> {
    match values.find(field) {
        Some(&Value::String(ref value)) => {
            if value.is_empty() {
                // Allow empty values
                return Ok(None);
            }
            if Url::parse(value).is_ok() {
                return Ok(None);
            }
            Err(format!("The {} field must contain a properly formatted URL.",
                        field.last()
                            .unwrap()
                            .to_lowercase()
                            .replace("_", " ")))
        }
        None => {
            // Allow empty values
            Ok(None)
        }
        _ => {
            Err(format!("The {} field must contain a properly formatted URL.",
                        field.last()
                            .unwrap()
                            .to_lowercase()
                            .replace("_", " ")))
        }
    }
}
