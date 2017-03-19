extern crate iron_valid;
extern crate params;

use iron_valid::{Rule, validate};
use params::{Map, Value};
use std::collections::BTreeMap;

#[test]
fn test_filled_valid_boolean_true() {
    let mut params = Map::new();
    params.assign("filled", Value::Boolean(true)).ok();

    let mut rules = BTreeMap::new();
    rules.insert("filled", vec![Rule::Filled]);

    let result = validate(&rules, params);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().find(&["filled"]).unwrap(),
               &Value::Boolean(true));
}

#[test]
fn test_filled_valid_boolean_false() {
    let mut params = Map::new();
    params.assign("filled", Value::Boolean(false)).ok();

    let mut rules = BTreeMap::new();
    rules.insert("filled", vec![Rule::Filled]);

    let result = validate(&rules, params);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().find(&["filled"]).unwrap(),
               &Value::Boolean(false));
}

#[test]
fn test_filled_valid_string() {
    let mut params = Map::new();
    params.assign("filled", Value::String("true".to_owned())).ok();

    let mut rules = BTreeMap::new();
    rules.insert("filled", vec![Rule::Filled]);

    let result = validate(&rules, params);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().find(&["filled"]).unwrap(),
               &Value::String("true".to_owned()));
}

#[test]
fn test_filled_invalid_empty_string() {
    let mut params = Map::new();
    params.assign("filled", Value::String("".to_owned())).ok();

    let mut rules = BTreeMap::new();
    rules.insert("filled", vec![Rule::Filled]);

    let result = validate(&rules, params);

    assert!(result.is_err());
    assert_eq!(*result.unwrap_err().get("filled").unwrap(),
               vec!["The filled field must be filled.".to_owned()]);
}

#[test]
fn test_filled_valid_numeric() {
    let mut params = Map::new();
    params.assign("filled", Value::U64(1)).ok();

    let mut rules = BTreeMap::new();
    rules.insert("filled", vec![Rule::Filled]);

    let result = validate(&rules, params);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().find(&["filled"]).unwrap(), &Value::U64(1));
}

#[test]
fn test_filled_valid_zero() {
    let mut params = Map::new();
    params.assign("filled", Value::U64(0)).ok();

    let mut rules = BTreeMap::new();
    rules.insert("filled", vec![Rule::Filled]);

    let result = validate(&rules, params);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().find(&["filled"]).unwrap(), &Value::U64(0));
}

#[test]
fn test_filled_valid_array() {
    let mut params = Map::new();
    params.assign("filled", Value::Array(vec![Value::F64(42.0)])).ok();

    let mut rules = BTreeMap::new();
    rules.insert("filled", vec![Rule::Filled]);

    let result = validate(&rules, params);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().find(&["filled"]).unwrap(),
               &Value::Array(vec![Value::F64(42.0)]));
}

#[test]
fn test_filled_invalid_empty_array() {
    let mut params = Map::new();
    params.assign("filled", Value::Array(vec![])).ok();

    let mut rules = BTreeMap::new();
    rules.insert("filled", vec![Rule::Filled]);

    let result = validate(&rules, params);

    assert!(result.is_err());
    assert_eq!(*result.unwrap_err().get("filled").unwrap(),
               vec!["The filled field must be filled.".to_owned()]);
}

#[test]
fn test_filled_valid_object() {
    let mut params = Map::new();
    let mut items = Map::new();
    items.assign("foo", Value::U64(1)).ok();
    params.assign("filled", Value::Map(items.clone())).ok();

    let mut rules = BTreeMap::new();
    rules.insert("filled", vec![Rule::Filled]);

    let result = validate(&rules, params);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().find(&["filled"]).unwrap(),
               &Value::Map(items));
}

#[test]
fn test_filled_invalid_empty_object() {
    let mut params = Map::new();
    let items = Map::new();
    params.assign("filled", Value::Map(items.clone())).ok();

    let mut rules = BTreeMap::new();
    rules.insert("filled", vec![Rule::Filled]);

    let result = validate(&rules, params);

    assert!(result.is_err());
    assert_eq!(*result.unwrap_err().get("filled").unwrap(),
               vec!["The filled field must be filled.".to_owned()]);
}

#[test]
fn test_filled_valid_blank() {
    let params = Map::new();

    let mut rules = BTreeMap::new();
    rules.insert("filled", vec![Rule::Filled]);

    let result = validate(&rules, params);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().find(&["filled"]), None);
}

#[test]
fn test_filled_invalid_null() {
    let mut params = Map::new();
    params.assign("filled", Value::Null).ok();

    let mut rules = BTreeMap::new();
    rules.insert("filled", vec![Rule::Filled]);

    let result = validate(&rules, params);

    assert!(result.is_err());
    assert_eq!(*result.unwrap_err().get("filled").unwrap(),
               vec!["The filled field must be filled.".to_owned()]);
}

#[test]
fn test_filled_valid_nested() {
    let mut test = Map::new();
    test.assign("filled", Value::Boolean(true)).ok();
    let mut params = Map::new();
    params.assign("test", Value::Map(test)).ok();

    let mut rules = BTreeMap::new();
    rules.insert("test.filled", vec![Rule::Filled]);

    let result = validate(&rules, params);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().find(&["test", "filled"]).unwrap(),
               &Value::Boolean(true));
}
