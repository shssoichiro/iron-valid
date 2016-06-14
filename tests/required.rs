extern crate iron_valid;
extern crate params;

use iron_valid::{Rule, validate};
use params::{Map, Value};
use std::collections::BTreeMap;

#[test]
fn test_required_valid_boolean_true() {
    let mut params = Map::new();
    params.assign("required", Value::Boolean(true)).ok();

    let mut rules = BTreeMap::new();
    rules.insert("required", vec![Rule::Required]);

    let result = validate(rules, params);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().find(&["required"]).unwrap(),
               &Value::Boolean(true));
}

#[test]
fn test_required_valid_boolean_false() {
    let mut params = Map::new();
    params.assign("required", Value::Boolean(false)).ok();

    let mut rules = BTreeMap::new();
    rules.insert("required", vec![Rule::Required]);

    let result = validate(rules, params);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().find(&["required"]).unwrap(),
               &Value::Boolean(false));
}

#[test]
fn test_required_valid_string() {
    let mut params = Map::new();
    params.assign("required", Value::String("true".to_owned())).ok();

    let mut rules = BTreeMap::new();
    rules.insert("required", vec![Rule::Required]);

    let result = validate(rules, params);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().find(&["required"]).unwrap(),
               &Value::String("true".to_owned()));
}

#[test]
fn test_required_invalid_empty_string() {
    let mut params = Map::new();
    params.assign("required", Value::String("".to_owned())).ok();

    let mut rules = BTreeMap::new();
    rules.insert("required", vec![Rule::Required]);

    let result = validate(rules, params);

    assert!(result.is_err());
    assert_eq!(*result.unwrap_err().get("required").unwrap(),
               vec!["The required field is required.".to_owned()]);
}

#[test]
fn test_required_valid_numeric() {
    let mut params = Map::new();
    params.assign("required", Value::U64(1)).ok();

    let mut rules = BTreeMap::new();
    rules.insert("required", vec![Rule::Required]);

    let result = validate(rules, params);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().find(&["required"]).unwrap(), &Value::U64(1));
}

#[test]
fn test_required_valid_array() {
    let mut params = Map::new();
    params.assign("required", Value::Array(vec![Value::U64(1)])).ok();

    let mut rules = BTreeMap::new();
    rules.insert("required", vec![Rule::Required]);

    let result = validate(rules, params);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().find(&["required"]).unwrap(),
               &Value::Array(vec![Value::U64(1)]));
}

#[test]
fn test_required_invalid_array() {
    let mut params = Map::new();
    params.assign("required", Value::Array(vec![])).ok();

    let mut rules = BTreeMap::new();
    rules.insert("required", vec![Rule::Required]);

    let result = validate(rules, params);

    assert!(result.is_err());
    assert_eq!(*result.unwrap_err().get("required").unwrap(),
               vec!["The required field is required.".to_owned()]);
}

#[test]
fn test_required_valid_object() {
    let mut params = Map::new();
    let mut items = Map::new();
    items.assign("foo", Value::U64(1)).ok();
    params.assign("required", Value::Map(items.clone())).ok();

    let mut rules = BTreeMap::new();
    rules.insert("required", vec![Rule::Required]);

    let result = validate(rules, params);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().find(&["required"]).unwrap(),
               &Value::Map(items));
}

#[test]
fn test_required_invalid_object() {
    let mut params = Map::new();
    let items = Map::new();
    params.assign("required", Value::Map(items.clone())).ok();

    let mut rules = BTreeMap::new();
    rules.insert("required", vec![Rule::Required]);

    let result = validate(rules, params);

    assert!(result.is_err());
    assert_eq!(*result.unwrap_err().get("required").unwrap(),
               vec!["The required field is required.".to_owned()]);
}

#[test]
fn test_required_invalid_blank() {
    let params = Map::new();

    let mut rules = BTreeMap::new();
    rules.insert("required", vec![Rule::Required]);

    let result = validate(rules, params);

    assert!(result.is_err());
    assert_eq!(*result.unwrap_err().get("required").unwrap(),
               vec!["The required field is required.".to_owned()]);
}

#[test]
fn test_required_invalid_null() {
    let mut params = Map::new();
    params.assign("required", Value::Null).ok();

    let mut rules = BTreeMap::new();
    rules.insert("required", vec![Rule::Required]);

    let result = validate(rules, params);

    assert!(result.is_err());
    assert_eq!(*result.unwrap_err().get("required").unwrap(),
               vec!["The required field is required.".to_owned()]);
}
