use params::{Map, Value};

pub fn validate_distinct(values: &Map, field: &str) -> Result<Option<Value>, String> {
    match values.find(&[field]) {
        Some(&Value::Array(ref value)) => {
            let mut checked: Vec<Value> = Vec::with_capacity(value.len());
            for item in value {
                if checked.contains(item) {
                    return Err(format!("The {} field must not contain any duplicate values.",
                                       field));
                }
                checked.push(item.clone());
            }
            Ok(None)
        }
        _ => Ok(None),
    }
}
