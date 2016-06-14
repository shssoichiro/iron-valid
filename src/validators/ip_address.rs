use params::{Map, Value};
use std::net::{Ipv4Addr, Ipv6Addr};
use std::str::FromStr;

pub fn validate_ip_address(values: &Map, field: &str) -> Result<Option<Value>, String> {
    match values.find(&[field]) {
        Some(&Value::String(ref value)) => {
            if value.is_empty() {
                // Allow empty values
                return Ok(None);
            }
            if Ipv4Addr::from_str(value).is_ok() || Ipv6Addr::from_str(value).is_ok() {
                return Ok(None);
            }
            Err(format!("The {} field must contain a valid IP address.",
                        field.to_lowercase().replace("_", " ")))
        }
        None => {
            // Allow empty values
            Ok(None)
        }
        _ => {
            Err(format!("The {} field must contain a valid IP address.",
                        field.to_lowercase().replace("_", " ")))
        }
    }
}
