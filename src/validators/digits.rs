use params::{Map, Value};

pub fn validate_digits(values: &Map, field: &str, digits: usize) -> Result<Option<Value>, String> {
    match values.find(&[field]) {
        Some(&Value::String(ref value)) => {
            if value.is_empty() {
                // Allow empty values
                return Ok(None);
            }
            let positive = value.parse::<u64>();
            match positive {
                Ok(uvalue) => {
                    if value.len() == digits {
                        Ok(Some(Value::U64(uvalue)))
                    } else {
                        Err(format!("The {} field must be a number with {} digits.",
                                    field.to_lowercase().replace("_", " "),
                                    digits))
                    }
                }
                Err(_) => {
                    let negative = value.parse::<i64>();
                    match negative {
                        Ok(ivalue) => {
                            if value[1..].len() == digits {
                                Ok(Some(Value::I64(ivalue)))
                            } else {
                                Err(format!("The {} field must be a number with {} digits.",
                                            field.to_lowercase().replace("_", " "),
                                            digits))
                            }
                        }
                        Err(_) => {
                            let float = value.parse::<f64>();
                            match float {
                                Ok(fvalue) => {
                                    let whole = value.find('.').unwrap();
                                    if value[..whole].len() == digits {
                                        Ok(Some(Value::F64(fvalue)))
                                    } else {
                                        Err(format!("The {} field must be a number with {} digits.",
                                                    field.to_lowercase().replace("_", " "),
                                                    digits))
                                    }
                                }
                                Err(_) => {
                                    Err(format!("The {} field must be a number with {} digits.",
                                                field.to_lowercase().replace("_", " "),
                                                digits))
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
            if count == digits {
                Ok(None)
            } else {
                Err(format!("The {} field must be a number with {} digits.",
                            field.to_lowercase().replace("_", " "),
                            digits))
            }
        }
        Some(&Value::I64(ref value)) => {
            let mut count = 0;
            let mut value = value.abs();
            while value > 0 {
                value /= 10;
                count += 1;
            }
            if count == digits {
                Ok(None)
            } else {
                Err(format!("The {} field must be a number with {} digits.",
                            field.to_lowercase().replace("_", " "),
                            digits))
            }
        }
        Some(&Value::F64(ref value)) => {
            let mut count = 0;
            let mut value = value.abs().floor() as usize;
            while value > 0 {
                value /= 10;
                count += 1;
            }
            if count == digits {
                Ok(None)
            } else {
                Err(format!("The {} field must be a number with {} digits.",
                            field.to_lowercase().replace("_", " "),
                            digits))
            }
        }
        None => {
            // Allow empty values
            Ok(None)
        }
        _ => {
            Err(format!("The {} field must be a number with {} digits.",
                        field.to_lowercase().replace("_", " "),
                        digits))
        }
    }
}
