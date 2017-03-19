use params::{Map, Value};

pub fn validate_alpha_numeric(values: &Map, field: &[&str]) -> Result<Option<Value>, String> {
    match values.find(field) {
        Some(&Value::String(ref value)) => {
            if value.is_empty() {
                // Allow empty values
                return Ok(None);
            }
            if value.chars().filter(|c| !c.is_alphanumeric()).count() == 0 {
                return Ok(None);
            }
            Err(format!("The {} field may only contain alphanumeric characters.",
                        field.last()
                            .unwrap()
                            .to_lowercase()
                            .replace("_", " ")))
        }
        Some(&Value::U64(ref value)) => Ok(Some(Value::String(format!("{}", value)))),
        Some(&Value::I64(ref value)) if *value >= 0 => {
            Ok(Some(Value::String(format!("{}", value))))
        }
        None => {
            // Allow empty values
            Ok(None)
        }
        _ => {
            Err(format!("The {} field may only contain alphanumeric characters.",
                        field.last()
                            .unwrap()
                            .to_lowercase()
                            .replace("_", " ")))
        }
    }
}
