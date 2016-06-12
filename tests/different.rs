extern crate iron_valid;
extern crate params;

use iron_valid::{Rule, validate};
use params::{Map, Value};
use std::collections::BTreeMap;

#[test]
fn test_different_valid_string() {
    let mut params = Map::new();
    params.assign("different", Value::String("foo".to_owned())).ok();
    params.assign("other", Value::String("bar".to_owned())).ok();

    let mut rules = BTreeMap::new();
    rules.insert("different", vec![Rule::Different("other".to_owned())]);

    let result = validate(rules, params);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().find(&["different"]).unwrap(),
               &Value::String("foo".to_owned()));
}

#[test]
fn test_different_valid_empty_string() {
    let mut params = Map::new();
    params.assign("different", Value::String("".to_owned())).ok();
    params.assign("other", Value::String("".to_owned())).ok();

    let mut rules = BTreeMap::new();
    rules.insert("different", vec![Rule::Different("other".to_owned())]);

    let result = validate(rules, params);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().find(&["different"]).unwrap(),
               &Value::String("".to_owned()));
}

#[test]
fn test_different_invalid_string() {
    let mut params = Map::new();
    params.assign("different", Value::String("foo".to_owned())).ok();
    params.assign("other", Value::String("foo".to_owned())).ok();

    let mut rules = BTreeMap::new();
    rules.insert("different", vec![Rule::Different("other".to_owned())]);

    let result = validate(rules, params);

    assert!(result.is_err());
    assert_eq!(*result.unwrap_err().get("different").unwrap(),
               vec!["The different field must be different than the other field.".to_owned()]);
}

#[test]
fn test_different_valid_u64() {
    let mut params = Map::new();
    params.assign("different", Value::U64(42)).ok();
    params.assign("other", Value::U64(41)).ok();

    let mut rules = BTreeMap::new();
    rules.insert("different", vec![Rule::Different("other".to_owned())]);

    let result = validate(rules, params);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().find(&["different"]).unwrap(),
               &Value::U64(42));
}

#[test]
fn test_different_invalid_u64() {
    let mut params = Map::new();
    params.assign("different", Value::U64(42)).ok();
    params.assign("other", Value::U64(42)).ok();

    let mut rules = BTreeMap::new();
    rules.insert("different", vec![Rule::Different("other".to_owned())]);

    let result = validate(rules, params);

    assert!(result.is_err());
    assert_eq!(*result.unwrap_err().get("different").unwrap(),
               vec!["The different field must be different than the other field.".to_owned()]);
}

#[test]
fn test_different_valid_i64() {
    let mut params = Map::new();
    params.assign("different", Value::I64(42)).ok();
    params.assign("other", Value::I64(41)).ok();

    let mut rules = BTreeMap::new();
    rules.insert("different", vec![Rule::Different("other".to_owned())]);

    let result = validate(rules, params);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().find(&["different"]).unwrap(),
               &Value::I64(42));
}

#[test]
fn test_different_invalid_i64() {
    let mut params = Map::new();
    params.assign("different", Value::I64(42)).ok();
    params.assign("other", Value::I64(42)).ok();

    let mut rules = BTreeMap::new();
    rules.insert("different", vec![Rule::Different("other".to_owned())]);

    let result = validate(rules, params);

    assert!(result.is_err());
    assert_eq!(*result.unwrap_err().get("different").unwrap(),
               vec!["The different field must be different than the other field.".to_owned()]);
}

#[test]
fn test_different_valid_f64() {
    let mut params = Map::new();
    params.assign("different", Value::F64(42.0)).ok();
    params.assign("other", Value::F64(41.0)).ok();

    let mut rules = BTreeMap::new();
    rules.insert("different", vec![Rule::Different("other".to_owned())]);

    let result = validate(rules, params);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().find(&["different"]).unwrap(),
               &Value::F64(42.0));
}

#[test]
fn test_different_invalid_f64() {
    let mut params = Map::new();
    params.assign("different", Value::F64(42.0)).ok();
    params.assign("other", Value::F64(42.0)).ok();

    let mut rules = BTreeMap::new();
    rules.insert("different", vec![Rule::Different("other".to_owned())]);

    let result = validate(rules, params);

    assert!(result.is_err());
    assert_eq!(*result.unwrap_err().get("different").unwrap(),
               vec!["The different field must be different than the other field.".to_owned()]);
}

#[test]
fn test_different_valid_array() {
    let mut params = Map::new();
    params.assign("different", Value::Array(vec![Value::F64(42.0)])).ok();
    params.assign("other", Value::Array(vec![Value::F64(41.0)]))
        .ok();

    let mut rules = BTreeMap::new();
    rules.insert("different", vec![Rule::Different("other".to_owned())]);

    let result = validate(rules, params);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().find(&["different"]).unwrap(),
               &Value::Array(vec![Value::F64(42.0)]));
}

#[test]
fn test_different_valid_empty_array() {
    let mut params = Map::new();
    params.assign("different", Value::Array(vec![])).ok();
    params.assign("other", Value::Array(vec![])).ok();

    let mut rules = BTreeMap::new();
    rules.insert("different", vec![Rule::Different("other".to_owned())]);

    let result = validate(rules, params);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().find(&["different"]).unwrap(),
               &Value::Array(vec![]));
}

#[test]
fn test_different_invalid_array() {
    let mut params = Map::new();
    params.assign("different", Value::Array(vec![Value::F64(42.0)])).ok();
    params.assign("other", Value::Array(vec![Value::F64(42.0)]))
        .ok();

    let mut rules = BTreeMap::new();
    rules.insert("different", vec![Rule::Different("other".to_owned())]);

    let result = validate(rules, params);

    assert!(result.is_err());
    assert_eq!(*result.unwrap_err().get("different").unwrap(),
               vec!["The different field must be different than the other field.".to_owned()]);
}

#[test]
fn test_different_valid_object() {
    let mut params = Map::new();
    let mut items = Map::new();
    items.assign("foo", Value::U64(1)).ok();
    let mut other_items = Map::new();
    other_items.assign("foo", Value::U64(2)).ok();
    params.assign("different", Value::Map(items.clone())).ok();
    params.assign("other", Value::Map(other_items.clone())).ok();

    let mut rules = BTreeMap::new();
    rules.insert("different", vec![Rule::Different("other".to_owned())]);

    let result = validate(rules, params);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().find(&["different"]).unwrap(),
               &Value::Map(items));
}

#[test]
fn test_different_valid_empty_object() {
    let mut params = Map::new();
    let items = Map::new();
    params.assign("different", Value::Map(items.clone())).ok();
    params.assign("other", Value::Map(items.clone())).ok();

    let mut rules = BTreeMap::new();
    rules.insert("different", vec![Rule::Different("other".to_owned())]);

    let result = validate(rules, params);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().find(&["different"]).unwrap(),
               &Value::Map(items));
}

#[test]
fn test_different_invalid_object() {
    let mut params = Map::new();
    let mut items = Map::new();
    items.assign("foo", Value::U64(1)).ok();
    params.assign("different", Value::Map(items.clone())).ok();
    params.assign("other", Value::Map(items.clone())).ok();

    let mut rules = BTreeMap::new();
    rules.insert("different", vec![Rule::Different("other".to_owned())]);

    let result = validate(rules, params);

    assert!(result.is_err());
    assert_eq!(*result.unwrap_err().get("different").unwrap(),
               vec!["The different field must be different than the other field.".to_owned()]);
}

#[test]
fn test_different_valid_boolean() {
    let mut params = Map::new();
    params.assign("different", Value::Boolean(true)).ok();
    params.assign("other", Value::Boolean(false)).ok();

    let mut rules = BTreeMap::new();
    rules.insert("different", vec![Rule::Different("other".to_owned())]);

    let result = validate(rules, params);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().find(&["different"]).unwrap(),
               &Value::Boolean(true));
}

#[test]
fn test_different_invalid_boolean() {
    let mut params = Map::new();
    params.assign("different", Value::Boolean(true)).ok();
    params.assign("other", Value::Boolean(true)).ok();

    let mut rules = BTreeMap::new();
    rules.insert("different", vec![Rule::Different("other".to_owned())]);

    let result = validate(rules, params);

    assert!(result.is_err());
    assert_eq!(*result.unwrap_err().get("different").unwrap(),
               vec!["The different field must be different than the other field.".to_owned()]);
}

#[test]
fn test_different_valid_blank() {
    let params = Map::new();

    let mut rules = BTreeMap::new();
    rules.insert("different", vec![Rule::Different("other".to_owned())]);

    let result = validate(rules, params);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().find(&["different"]), None);
}

#[test]
fn test_different_invalid_null() {
    let mut params = Map::new();
    params.assign("different", Value::Null).ok();

    let mut rules = BTreeMap::new();
    rules.insert("different", vec![Rule::Different("other".to_owned())]);

    let result = validate(rules, params);

    assert!(result.is_err());
    assert_eq!(*result.unwrap_err().get("different").unwrap(),
               vec!["The different field must be different than the other field.".to_owned()]);
}
