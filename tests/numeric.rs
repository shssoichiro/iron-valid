extern crate iron_valid;
extern crate params;

use iron_valid::{Rule, validate};
use params::{Map, Value};
use std::collections::BTreeMap;

#[test]
fn test_numeric_valid_string() {
    let mut params = Map::new();
    params.assign("numeric", Value::String("3".to_owned())).ok();

    let mut rules = BTreeMap::new();
    rules.insert("numeric", vec![Rule::Numeric]);

    let result = validate(rules, params);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().find(&["numeric"]).unwrap(), &Value::U64(3));
}

#[test]
fn test_numeric_valid_empty_string() {
    let mut params = Map::new();
    params.assign("numeric", Value::String("".to_owned())).ok();

    let mut rules = BTreeMap::new();
    rules.insert("numeric", vec![Rule::Numeric]);

    let result = validate(rules, params);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().find(&["numeric"]).unwrap(),
               &Value::String("".to_owned()));
}

#[test]
fn test_numeric_invalid_string() {
    let mut params = Map::new();
    params.assign("numeric", Value::String("foo".to_owned())).ok();

    let mut rules = BTreeMap::new();
    rules.insert("numeric", vec![Rule::Numeric]);

    let result = validate(rules, params);

    assert!(result.is_err());
    assert_eq!(*result.unwrap_err().get("numeric").unwrap(),
               vec!["The numeric field must be numeric.".to_owned()]);
}

#[test]
fn test_numeric_valid_u64() {
    let mut params = Map::new();
    params.assign("numeric", Value::U64(3)).ok();

    let mut rules = BTreeMap::new();
    rules.insert("numeric", vec![Rule::Numeric]);

    let result = validate(rules, params);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().find(&["numeric"]).unwrap(), &Value::U64(3));
}

#[test]
fn test_numeric_valid_i64() {
    let mut params = Map::new();
    params.assign("numeric", Value::I64(-3)).ok();

    let mut rules = BTreeMap::new();
    rules.insert("numeric", vec![Rule::Numeric]);

    let result = validate(rules, params);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().find(&["numeric"]).unwrap(), &Value::I64(-3));
}

#[test]
fn test_numeric_valid_f64() {
    let mut params = Map::new();
    params.assign("numeric", Value::F64(10.0)).ok();

    let mut rules = BTreeMap::new();
    rules.insert("numeric", vec![Rule::Numeric]);

    let result = validate(rules, params);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().find(&["numeric"]).unwrap(),
               &Value::F64(10.0));
}

#[test]
fn test_numeric_invalid_array() {
    let mut params = Map::new();
    params.assign("numeric",
                Value::Array(vec![Value::U64(1), Value::U64(2), Value::U64(3)]))
        .ok();

    let mut rules = BTreeMap::new();
    rules.insert("numeric", vec![Rule::Numeric]);

    let result = validate(rules, params);

    assert!(result.is_err());
    assert_eq!(*result.unwrap_err().get("numeric").unwrap(),
               vec!["The numeric field must be numeric.".to_owned()]);
}

#[test]
fn test_numeric_invalid_object() {
    let mut params = Map::new();
    let mut items = Map::new();
    items.assign("foo", Value::U64(1)).ok();
    items.assign("bar", Value::U64(2)).ok();
    items.assign("baz", Value::U64(3)).ok();
    params.assign("numeric", Value::Map(items.clone())).ok();

    let mut rules = BTreeMap::new();
    rules.insert("numeric", vec![Rule::Numeric]);

    let result = validate(rules, params);

    assert!(result.is_err());
    assert_eq!(*result.unwrap_err().get("numeric").unwrap(),
               vec!["The numeric field must be numeric.".to_owned()]);
}

#[test]
fn test_numeric_valid_blank() {
    let params = Map::new();

    let mut rules = BTreeMap::new();
    rules.insert("numeric", vec![Rule::Numeric]);

    let result = validate(rules, params);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().find(&["numeric"]), None);
}

#[test]
fn test_numeric_invalid_null() {
    let mut params = Map::new();
    params.assign("numeric", Value::Null).ok();

    let mut rules = BTreeMap::new();
    rules.insert("numeric", vec![Rule::Numeric]);

    let result = validate(rules, params);

    assert!(result.is_err());
    assert_eq!(*result.unwrap_err().get("numeric").unwrap(),
               vec!["The numeric field must be numeric.".to_owned()]);
}
