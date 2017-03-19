use params::{Map, Value};

pub fn validate_digits_between(values: &Map,
                               field: &[&str],
                               min: usize,
                               max: usize)
                               -> Result<Option<Value>, String> {
    match values.find(field) {
        Some(&Value::String(ref value)) => {
            if value.is_empty() {
                // Allow empty values
                return Ok(None);
            }
            let positive = value.parse::<u64>();
            match positive {
                Ok(uvalue) => {
                    let len = value.len();
                    if len >= min && len <= max {
                        Ok(Some(Value::U64(uvalue)))
                    } else {
                        Err(format!("The {} field must be a number with between {} and {} digits.",
                                    field.last()
                                        .unwrap()
                                        .to_lowercase()
                                        .replace("_", " "),
                                    min,
                                    max))
                    }
                }
                Err(_) => {
                    let negative = value.parse::<i64>();
                    match negative {
                        Ok(ivalue) => {
                            let len = value[1..].len();
                            if len >= min && len <= max {
                                Ok(Some(Value::I64(ivalue)))
                            } else {
                                Err(format!("The {} field must be a number with between {} and {} digits.",
                                            field.last()
                                                .unwrap()
                                                .to_lowercase()
                                                .replace("_", " "),
                                            min,
                                            max))
                            }
                        }
                        Err(_) => {
                            let float = value.parse::<f64>();
                            match float {
                                Ok(fvalue) => {
                                    let whole = value.find('.').unwrap();
                                    let len = value[..whole].len();
                                    if len >= min && len <= max {
                                        Ok(Some(Value::F64(fvalue)))
                                    } else {
                                        Err(format!("The {} field must be a number with between {} and {} digits.",
                                                    field.last()
                                                        .unwrap()
                                                        .to_lowercase()
                                                        .replace("_", " "),
                                                    min,
                                                    max))
                                    }
                                }
                                Err(_) => {
                                    Err(format!("The {} field must be a number with between {} and {} digits.",
                                                field.last()
                                                    .unwrap()
                                                    .to_lowercase()
                                                    .replace("_", " "),
                                                min,
                                                max))
                                }
                            }
                        }
                    }
                }
            }
        }
        Some(&Value::U64(ref value)) => {
            let mut count = 0;
            let mut value = *value;
            while value > 0 {
                value /= 10;
                count += 1;
            }
            if count >= min && count <= max {
                Ok(None)
            } else {
                Err(format!("The {} field must be a number with between {} and {} digits.",
                            field.last()
                                .unwrap()
                                .to_lowercase()
                                .replace("_", " "),
                            min,
                            max))
            }
        }
        Some(&Value::I64(ref value)) => {
            let mut count = 0;
            let mut value = value.abs();
            while value > 0 {
                value /= 10;
                count += 1;
            }
            if count >= min && count <= max {
                Ok(None)
            } else {
                Err(format!("The {} field must be a number with between {} and {} digits.",
                            field.last()
                                .unwrap()
                                .to_lowercase()
                                .replace("_", " "),
                            min,
                            max))
            }
        }
        Some(&Value::F64(ref value)) => {
            let mut count = 0;
            let mut value = value.abs().floor() as usize;
            while value > 0 {
                value /= 10;
                count += 1;
            }
            if count >= min && count <= max {
                Ok(None)
            } else {
                Err(format!("The {} field must be a number with between {} and {} digits.",
                            field.last()
                                .unwrap()
                                .to_lowercase()
                                .replace("_", " "),
                            min,
                            max))
            }
        }
        None => {
            // Allow empty values
            Ok(None)
        }
        _ => {
            Err(format!("The {} field must be a number with between {} and {} digits.",
                        field.last()
                            .unwrap()
                            .to_lowercase()
                            .replace("_", " "),
                        min,
                        max))
        }
    }
}
