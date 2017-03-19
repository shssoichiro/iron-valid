use params::{Map, Value};

pub fn validate_boolean(values: &Map, field: &[&str]) -> Result<Option<Value>, String> {
    match values.find(field) {
        Some(&Value::Boolean(_)) => Ok(None),
        Some(&Value::String(ref value)) => {
            let value = value.to_lowercase();
            if value == "true" || value == "1" {
                return Ok(Some(Value::Boolean(true)));
            }
            if value == "false" || value == "0" {
                return Ok(Some(Value::Boolean(false)));
            }
            Err(format!("The {} field must be a boolean.",
                        field.last()
                            .unwrap()
                            .to_lowercase()
                            .replace("_", " ")))
        }
        Some(&Value::U64(ref value)) => {
            if *value == 1 {
                return Ok(Some(Value::Boolean(true)));
            }
            if *value == 0 {
                return Ok(Some(Value::Boolean(false)));
            }
            Err(format!("The {} field must be a boolean.",
                        field.last()
                            .unwrap()
                            .to_lowercase()
                            .replace("_", " ")))
        }
        Some(&Value::I64(ref value)) => {
            if *value == 1 {
                return Ok(Some(Value::Boolean(true)));
            }
            if *value == 0 {
                return Ok(Some(Value::Boolean(false)));
            }
            Err(format!("The {} field must be a boolean.",
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
            Err(format!("The {} field must be a boolean.",
                        field.last()
                            .unwrap()
                            .to_lowercase()
                            .replace("_", " ")))
        }
    }
}
