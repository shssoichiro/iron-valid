extern crate iron_valid;
extern crate params;

use iron_valid::{Rule, validate};
use params::{Map, Value};
use std::collections::BTreeMap;

#[test]
fn test_string_valid_numeric() {
    let mut params = Map::new();
    params.assign("string", Value::String("3".to_owned())).ok();

    let mut rules = BTreeMap::new();
    rules.insert("string", vec![Rule::String]);

    let result = validate(rules, params);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().find(&["string"]).unwrap(),
               &Value::String("3".to_owned()));
}

#[test]
fn test_string_valid_empty_string() {
    let mut params = Map::new();
    params.assign("string", Value::String("".to_owned())).ok();

    let mut rules = BTreeMap::new();
    rules.insert("string", vec![Rule::String]);

    let result = validate(rules, params);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().find(&["string"]).unwrap(),
               &Value::String("".to_owned()));
}

#[test]
fn test_string_valid_string() {
    let mut params = Map::new();
    params.assign("string", Value::String("foo".to_owned())).ok();

    let mut rules = BTreeMap::new();
    rules.insert("string", vec![Rule::String]);

    let result = validate(rules, params);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().find(&["string"]).unwrap(),
               &Value::String("foo".to_owned()));
}

#[test]
fn test_string_invalid_u64() {
    let mut params = Map::new();
    params.assign("string", Value::U64(3)).ok();

    let mut rules = BTreeMap::new();
    rules.insert("string", vec![Rule::String]);

    let result = validate(rules, params);

    assert!(result.is_err());
    assert_eq!(*result.unwrap_err().get("string").unwrap(),
               vec!["The string field must be a string.".to_owned()]);
}

#[test]
fn test_string_invalid_i64() {
    let mut params = Map::new();
    params.assign("string", Value::I64(-3)).ok();

    let mut rules = BTreeMap::new();
    rules.insert("string", vec![Rule::String]);

    let result = validate(rules, params);

    assert!(result.is_err());
    assert_eq!(*result.unwrap_err().get("string").unwrap(),
               vec!["The string field must be a string.".to_owned()]);
}

#[test]
fn test_string_invalid_f64() {
    let mut params = Map::new();
    params.assign("string", Value::F64(10.0)).ok();

    let mut rules = BTreeMap::new();
    rules.insert("string", vec![Rule::String]);

    let result = validate(rules, params);

    assert!(result.is_err());
    assert_eq!(*result.unwrap_err().get("string").unwrap(),
               vec!["The string field must be a string.".to_owned()]);
}

#[test]
fn test_string_invalid_array() {
    let mut params = Map::new();
    params.assign("string",
                Value::Array(vec![Value::U64(1), Value::U64(2), Value::U64(3)]))
        .ok();

    let mut rules = BTreeMap::new();
    rules.insert("string", vec![Rule::String]);

    let result = validate(rules, params);

    assert!(result.is_err());
    assert_eq!(*result.unwrap_err().get("string").unwrap(),
               vec!["The string field must be a string.".to_owned()]);
}

#[test]
fn test_string_invalid_object() {
    let mut params = Map::new();
    let mut items = Map::new();
    items.assign("foo", Value::U64(1)).ok();
    items.assign("bar", Value::U64(2)).ok();
    items.assign("baz", Value::U64(3)).ok();
    params.assign("string", Value::Map(items.clone())).ok();

    let mut rules = BTreeMap::new();
    rules.insert("string", vec![Rule::String]);

    let result = validate(rules, params);

    assert!(result.is_err());
    assert_eq!(*result.unwrap_err().get("string").unwrap(),
               vec!["The string field must be a string.".to_owned()]);
}

#[test]
fn test_string_valid_blank() {
    let params = Map::new();

    let mut rules = BTreeMap::new();
    rules.insert("string", vec![Rule::String]);

    let result = validate(rules, params);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().find(&["string"]), None);
}

#[test]
fn test_string_invalid_null() {
    let mut params = Map::new();
    params.assign("string", Value::Null).ok();

    let mut rules = BTreeMap::new();
    rules.insert("string", vec![Rule::String]);

    let result = validate(rules, params);

    assert!(result.is_err());
    assert_eq!(*result.unwrap_err().get("string").unwrap(),
               vec!["The string field must be a string.".to_owned()]);
}
