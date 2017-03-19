extern crate iron_valid;
extern crate params;

use iron_valid::{Rule, validate};
use params::{Map, Value};
use std::collections::BTreeMap;

#[test]
fn test_array_valid_array() {
    let mut params = Map::new();
    params.assign("array", Value::Array(vec![Value::U64(1)])).ok();

    let mut rules = BTreeMap::new();
    rules.insert("array", vec![Rule::Array]);

    let result = validate(&rules, params);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().find(&["array"]).unwrap(),
               &Value::Array(vec![Value::U64(1)]));
}

#[test]
fn test_array_invalid_string() {
    let mut params = Map::new();
    params.assign("array", Value::String("[1]".to_owned())).ok();

    let mut rules = BTreeMap::new();
    rules.insert("array", vec![Rule::Array]);

    let result = validate(&rules, params);

    assert!(result.is_err());
    assert_eq!(*result.unwrap_err().get("array").unwrap(),
               vec!["The array field must be an array.".to_owned()]);
}

#[test]
fn test_array_invalid_numeric() {
    let mut params = Map::new();
    params.assign("array", Value::U64(42)).ok();

    let mut rules = BTreeMap::new();
    rules.insert("array", vec![Rule::Array]);

    let result = validate(&rules, params);

    assert!(result.is_err());
    assert_eq!(*result.unwrap_err().get("array").unwrap(),
               vec!["The array field must be an array.".to_owned()]);
}

#[test]
fn test_array_valid_empty() {
    let mut params = Map::new();
    params.assign("array", Value::Array(vec![])).ok();

    let mut rules = BTreeMap::new();
    rules.insert("array", vec![Rule::Array]);

    let result = validate(&rules, params);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().find(&["array"]).unwrap(),
               &Value::Array(vec![]));
}

#[test]
fn test_array_valid_blank() {
    let params = Map::new();

    let mut rules = BTreeMap::new();
    rules.insert("array", vec![Rule::Array]);

    let result = validate(&rules, params);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().find(&["array"]), None);
}

#[test]
fn test_array_invalid_null() {
    let mut params = Map::new();
    params.assign("array", Value::Null).ok();

    let mut rules = BTreeMap::new();
    rules.insert("array", vec![Rule::Array]);

    let result = validate(&rules, params);

    assert!(result.is_err());
    assert_eq!(*result.unwrap_err().get("array").unwrap(),
               vec!["The array field must be an array.".to_owned()]);
}

#[test]
fn test_array_valid_nested() {
    let mut test = Map::new();
    test.assign("array", Value::Array(vec![Value::U64(1)])).ok();
    let mut params = Map::new();
    params.assign("test", Value::Map(test)).ok();

    let mut rules = BTreeMap::new();
    rules.insert("test.array", vec![Rule::Array]);

    let result = validate(&rules, params);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().find(&["test", "array"]).unwrap(),
               &Value::Array(vec![Value::U64(1)]));
}
