extern crate iron_valid;
extern crate params;

use iron_valid::{Rule, validate};
use params::{Map, Value};
use std::collections::BTreeMap;

#[test]
fn test_boolean_valid_boolean_true() {
    let mut params = Map::new();
    params.assign("boolean", Value::Boolean(true)).ok();

    let mut rules = BTreeMap::new();
    rules.insert("boolean", vec![Rule::Boolean]);

    let result = validate(rules, params);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().find(&["boolean"]).unwrap(),
               &Value::Boolean(true));
}

#[test]
fn test_boolean_valid_boolean_false() {
    let mut params = Map::new();
    params.assign("boolean", Value::Boolean(false)).ok();

    let mut rules = BTreeMap::new();
    rules.insert("boolean", vec![Rule::Boolean]);

    let result = validate(rules, params);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().find(&["boolean"]).unwrap(),
               &Value::Boolean(false));
}

#[test]
fn test_boolean_valid_string() {
    let mut params = Map::new();
    params.assign("boolean", Value::String("true".to_owned())).ok();

    let mut rules = BTreeMap::new();
    rules.insert("boolean", vec![Rule::Boolean]);

    let result = validate(rules, params);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().find(&["boolean"]).unwrap(),
               &Value::Boolean(true));
}

#[test]
fn test_boolean_invalid_string() {
    let mut params = Map::new();
    params.assign("boolean", Value::String("nope".to_owned())).ok();

    let mut rules = BTreeMap::new();
    rules.insert("boolean", vec![Rule::Boolean]);

    let result = validate(rules, params);

    assert!(result.is_err());
    assert_eq!(*result.unwrap_err().get("boolean").unwrap(),
               vec!["The boolean field must be a boolean.".to_owned()]);
}

#[test]
fn test_boolean_valid_numeric() {
    let mut params = Map::new();
    params.assign("boolean", Value::U64(1)).ok();

    let mut rules = BTreeMap::new();
    rules.insert("boolean", vec![Rule::Boolean]);

    let result = validate(rules, params);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().find(&["boolean"]).unwrap(),
               &Value::Boolean(true));
}

#[test]
fn test_boolean_invalid_numeric() {
    let mut params = Map::new();
    params.assign("boolean", Value::U64(42)).ok();

    let mut rules = BTreeMap::new();
    rules.insert("boolean", vec![Rule::Boolean]);

    let result = validate(rules, params);

    assert!(result.is_err());
    assert_eq!(*result.unwrap_err().get("boolean").unwrap(),
               vec!["The boolean field must be a boolean.".to_owned()]);
}

#[test]
fn test_boolean_valid_blank() {
    let params = Map::new();

    let mut rules = BTreeMap::new();
    rules.insert("boolean", vec![Rule::Boolean]);

    let result = validate(rules, params);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().find(&["boolean"]), None);
}

#[test]
fn test_boolean_invalid_null() {
    let mut params = Map::new();
    params.assign("boolean", Value::Null).ok();

    let mut rules = BTreeMap::new();
    rules.insert("boolean", vec![Rule::Boolean]);

    let result = validate(rules, params);

    assert!(result.is_err());
    assert_eq!(*result.unwrap_err().get("boolean").unwrap(),
               vec!["The boolean field must be a boolean.".to_owned()]);
}
