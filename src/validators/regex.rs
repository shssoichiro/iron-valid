use regex::Regex;

use params::{Map, Value};

pub fn validate_regex(values: &Map,
                      field: &[&str],
                      pattern: &str)
                      -> Result<Option<Value>, String> {
    let pattern = Regex::new(pattern).expect("Invalid pattern passed into Regex validator");

    let value = match values.find(field) {
        Some(&Value::String(ref value)) => value.clone(),
        Some(&Value::U64(ref value)) => format!("{}", *value),
        Some(&Value::I64(ref value)) => format!("{}", *value),
        Some(&Value::F64(ref value)) => format!("{}", *value),
        Some(&Value::Boolean(ref value)) => format!("{}", *value),
        None => {
            // Allow empty values
            return Ok(None);
        }
        _ => {
            return Err(format!("The {} field must match the pattern \"{}\".",
                               field.last()
                                   .unwrap()
                                   .to_lowercase()
                                   .replace("_", " "),
                               pattern));
        }
    };

    if pattern.is_match(&value) {
        Ok(Some(Value::String(value)))
    } else {
        Err(format!("The {} field must match the pattern \"{}\".",
                    field.last()
                        .unwrap()
                        .to_lowercase()
                        .replace("_", " "),
                    pattern))
    }
}
