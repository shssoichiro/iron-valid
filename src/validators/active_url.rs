use dns_lookup;

use params::{Map, Value};

pub fn validate_active_url(values: &Map, field: &str) -> Result<Option<Value>, String> {
    match values.find(&[field]) {
        Some(&Value::String(ref value)) => {
            if value.is_empty() {
                // Allow empty values
                return Ok(None);
            }
            if dns_lookup::lookup_host(value).is_ok() {
                Ok(None)
            } else {
                Err(format!("The {} field must contain a valid, active domain name.",
                            field.to_lowercase().replace("_", " ")))
            }
        }
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
