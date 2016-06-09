use dns_lookup;
use params::{Map, Value};

pub fn validate_active_url(values: &Map, field: &str) -> Result<Option<Value>, String> {
    match values.find(&[field]) {
        Some(&Value::String(ref value)) => {
            if value.is_empty() {
                // Allow empty values
                return Ok(None);
            }
            if let Ok(_) = dns_lookup::lookup_host(value) {
                Ok(None)
            } else {
                Err(format!("The {} field must contain a valid, active domain name.",
                            field.to_lowercase().replace("_", " ")))
            }
        }
        Some(&Value::Null) |
        None => {
            // Allow empty values
            Ok(None)
        }
        _ => {
            Err(format!("The {} field must contain a valid, active domain name.",
                        field.to_lowercase().replace("_", " ")))
        }
    }
}