use params::{Map, Value};
use regex::Regex;

pub fn validate_alpha(values: &Map, field: &str) -> Result<Option<Value>, String> {
    lazy_static! {
        static ref ALPHA_REGEX: Regex = Regex::new(r"^[A-Za-z]+$").unwrap();
    }

    match values.find(&[field]) {
        Some(&Value::String(ref value)) => {
            if value.is_empty() {
                // Allow empty values
                return Ok(None);
            }
            if ALPHA_REGEX.is_match(value) {
                return Ok(None);
            }
            Err(format!("The {} field must contain only alphabetic characters.",
                        field.to_lowercase().replace("_", " ")))
        }
        Some(&Value::Null) |
        None => {
            // Allow empty values
            Ok(None)
        }
        _ => {
            Err(format!("The {} field must contain only alphabetic characters.",
                        field.to_lowercase().replace("_", " ")))
        }
    }
}
