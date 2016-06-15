use params::{Map, Value};

pub fn validate_required_without_all(values: &Map,
                                     field: &str,
                                     others: &[&str])
                                     -> Result<Option<Value>, String> {
    let mut required = true;
    for other in others {
        let current = match values.find(&[other]) {
            None |
            Some(&Value::Null) => true,
            Some(&Value::String(ref value)) if value.is_empty() => true,
            Some(&Value::Array(ref value)) if value.is_empty() => true,
            Some(&Value::Map(ref value)) if value.is_empty() => true,
            _ => false,
        };
        if !current {
            required = false;
            break;
        }
    }

    if required {
        match values.find(&[field]) {
            Some(&Value::String(ref value)) if value.is_empty() => {
                Err(format!("The {} field is required.",
                            field.to_lowercase().replace("_", " ")))
            }
            Some(&Value::Array(ref value)) if value.is_empty() => {
                Err(format!("The {} field is required.",
                            field.to_lowercase().replace("_", " ")))
            }
            Some(&Value::Map(ref value)) if value.is_empty() => {
                Err(format!("The {} field is required.",
                            field.to_lowercase().replace("_", " ")))
            }
            Some(&Value::Null) |
            None => {
                Err(format!("The {} field is required.",
                            field.to_lowercase().replace("_", " ")))
            }
            _ => Ok(None),
        }
    } else {
        Ok(None)
    }
}
