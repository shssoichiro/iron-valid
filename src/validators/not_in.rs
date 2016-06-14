use params::{Map, Value};

pub fn validate_not_in(values: &Map,
                       field: &str,
                       options: &[Value])
                       -> Result<Option<Value>, String> {
    match values.find(&[field]) {
        Some(&Value::String(ref value)) if value.is_empty() => Ok(None),
        Some(&Value::Array(ref value)) if value.is_empty() => Ok(None),
        Some(&Value::Map(ref value)) if value.is_empty() => Ok(None),
        Some(value) => {
            if options.contains(value) {
                Err(format!("The {} field must not be among the options: {:?}.",
                            field.to_lowercase().replace("_", " "),
                            options))
            } else {
                Ok(None)
            }
        }
        None => Ok(None),
    }
}
