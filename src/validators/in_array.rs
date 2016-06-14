use params::{Map, Value};

pub fn validate_in_array(values: &Map, field: &str, other: &str) -> Result<Option<Value>, String> {
    let options = match values.find(&[other]) {
        Some(&Value::Array(ref value)) => value,
        _ => {
            return Err(format!("The {} field must be one of the values in the {} field.",
                               field.to_lowercase().replace("_", " "),
                               other.to_lowercase().replace("_", " ")));
        }
    };

    match values.find(&[field]) {
        Some(&Value::String(ref value)) if value.is_empty() => Ok(None),
        Some(&Value::Array(ref value)) if value.is_empty() => Ok(None),
        Some(&Value::Map(ref value)) if value.is_empty() => Ok(None),
        Some(value) => {
            if options.contains(value) {
                Ok(None)
            } else {
                Err(format!("The {} field must be one of the values in the {} field.",
                            field.to_lowercase().replace("_", " "),
                            other.to_lowercase().replace("_", " ")))
            }
        }
        None => Ok(None),
    }
}
