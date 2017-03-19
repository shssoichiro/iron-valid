extern crate iron_valid;
extern crate params;

use iron_valid::{Rule, validate};
use params::{Map, Value};
use std::collections::BTreeMap;

#[test]
fn test_same_valid_string() {
    let mut params = Map::new();
    params.assign("same", Value::String("foo".to_owned())).ok();
    params.assign("other", Value::String("foo".to_owned())).ok();

    let mut rules = BTreeMap::new();
    rules.insert("same", vec![Rule::Same("other")]);

    let result = validate(&rules, params);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().find(&["same"]).unwrap(),
               &Value::String("foo".to_owned()));
}

#[test]
fn test_same_valid_empty_string() {
    let mut params = Map::new();
    params.assign("same", Value::String("".to_owned())).ok();
    params.assign("other", Value::String("".to_owned())).ok();

    let mut rules = BTreeMap::new();
    rules.insert("same", vec![Rule::Same("other")]);

    let result = validate(&rules, params);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().find(&["same"]).unwrap(),
               &Value::String("".to_owned()));
}

#[test]
fn test_same_invalid_string() {
    let mut params = Map::new();
    params.assign("same", Value::String("foo".to_owned())).ok();
    params.assign("other", Value::String("bar".to_owned())).ok();

    let mut rules = BTreeMap::new();
    rules.insert("same", vec![Rule::Same("other")]);

    let result = validate(&rules, params);

    assert!(result.is_err());
    assert_eq!(*result.unwrap_err().get("same").unwrap(),
               vec!["The same field must match the other field.".to_owned()]);
}

#[test]
fn test_same_valid_u64() {
    let mut params = Map::new();
    params.assign("same", Value::U64(42)).ok();
    params.assign("other", Value::U64(42)).ok();

    let mut rules = BTreeMap::new();
    rules.insert("same", vec![Rule::Same("other")]);

    let result = validate(&rules, params);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().find(&["same"]).unwrap(), &Value::U64(42));
}

#[test]
fn test_same_invalid_u64() {
    let mut params = Map::new();
    params.assign("same", Value::U64(42)).ok();
    params.assign("other", Value::U64(41)).ok();

    let mut rules = BTreeMap::new();
    rules.insert("same", vec![Rule::Same("other")]);

    let result = validate(&rules, params);

    assert!(result.is_err());
    assert_eq!(*result.unwrap_err().get("same").unwrap(),
               vec!["The same field must match the other field.".to_owned()]);
}

#[test]
fn test_same_valid_i64() {
    let mut params = Map::new();
    params.assign("same", Value::I64(42)).ok();
    params.assign("other", Value::I64(42)).ok();

    let mut rules = BTreeMap::new();
    rules.insert("same", vec![Rule::Same("other")]);

    let result = validate(&rules, params);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().find(&["same"]).unwrap(), &Value::I64(42));
}

#[test]
fn test_same_invalid_i64() {
    let mut params = Map::new();
    params.assign("same", Value::I64(42)).ok();
    params.assign("other", Value::I64(41)).ok();

    let mut rules = BTreeMap::new();
    rules.insert("same", vec![Rule::Same("other")]);

    let result = validate(&rules, params);

    assert!(result.is_err());
    assert_eq!(*result.unwrap_err().get("same").unwrap(),
               vec!["The same field must match the other field.".to_owned()]);
}

#[test]
fn test_same_valid_f64() {
    let mut params = Map::new();
    params.assign("same", Value::F64(42.0)).ok();
    params.assign("other", Value::F64(42.0)).ok();

    let mut rules = BTreeMap::new();
    rules.insert("same", vec![Rule::Same("other")]);

    let result = validate(&rules, params);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().find(&["same"]).unwrap(), &Value::F64(42.0));
}

#[test]
fn test_same_invalid_f64() {
    let mut params = Map::new();
    params.assign("same", Value::F64(42.0)).ok();
    params.assign("other", Value::F64(41.0)).ok();

    let mut rules = BTreeMap::new();
    rules.insert("same", vec![Rule::Same("other")]);

    let result = validate(&rules, params);

    assert!(result.is_err());
    assert_eq!(*result.unwrap_err().get("same").unwrap(),
               vec!["The same field must match the other field.".to_owned()]);
}

#[test]
fn test_same_valid_array() {
    let mut params = Map::new();
    params.assign("same", Value::Array(vec![Value::F64(42.0)])).ok();
    params.assign("other", Value::Array(vec![Value::F64(42.0)])).ok();

    let mut rules = BTreeMap::new();
    rules.insert("same", vec![Rule::Same("other")]);

    let result = validate(&rules, params);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().find(&["same"]).unwrap(),
               &Value::Array(vec![Value::F64(42.0)]));
}

#[test]
fn test_same_valid_empty_array() {
    let mut params = Map::new();
    params.assign("same", Value::Array(vec![])).ok();
    params.assign("other", Value::Array(vec![])).ok();

    let mut rules = BTreeMap::new();
    rules.insert("same", vec![Rule::Same("other")]);

    let result = validate(&rules, params);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().find(&["same"]).unwrap(),
               &Value::Array(vec![]));
}

#[test]
fn test_same_invalid_array() {
    let mut params = Map::new();
    params.assign("same", Value::Array(vec![Value::F64(42.0)])).ok();
    params.assign("other", Value::Array(vec![Value::F64(41.0)])).ok();

    let mut rules = BTreeMap::new();
    rules.insert("same", vec![Rule::Same("other")]);

    let result = validate(&rules, params);

    assert!(result.is_err());
    assert_eq!(*result.unwrap_err().get("same").unwrap(),
               vec!["The same field must match the other field.".to_owned()]);
}

#[test]
fn test_same_valid_object() {
    let mut params = Map::new();
    let mut items = Map::new();
    items.assign("foo", Value::U64(1)).ok();
    params.assign("same", Value::Map(items.clone())).ok();
    params.assign("other", Value::Map(items.clone())).ok();

    let mut rules = BTreeMap::new();
    rules.insert("same", vec![Rule::Same("other")]);

    let result = validate(&rules, params);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().find(&["same"]).unwrap(), &Value::Map(items));
}

#[test]
fn test_same_valid_empty_object() {
    let mut params = Map::new();
    let items = Map::new();
    params.assign("same", Value::Map(items.clone())).ok();
    params.assign("other", Value::Map(items.clone())).ok();

    let mut rules = BTreeMap::new();
    rules.insert("same", vec![Rule::Same("other")]);

    let result = validate(&rules, params);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().find(&["same"]).unwrap(), &Value::Map(items));
}

#[test]
fn test_same_invalid_object() {
    let mut params = Map::new();
    let mut items = Map::new();
    let empty_items = items.clone();
    items.assign("foo", Value::U64(1)).ok();
    params.assign("same", Value::Map(items.clone())).ok();
    params.assign("other", Value::Map(empty_items.clone())).ok();

    let mut rules = BTreeMap::new();
    rules.insert("same", vec![Rule::Same("other")]);

    let result = validate(&rules, params);

    assert!(result.is_err());
    assert_eq!(*result.unwrap_err().get("same").unwrap(),
               vec!["The same field must match the other field.".to_owned()]);
}

#[test]
fn test_same_valid_boolean() {
    let mut params = Map::new();
    params.assign("same", Value::Boolean(true)).ok();
    params.assign("other", Value::Boolean(true)).ok();

    let mut rules = BTreeMap::new();
    rules.insert("same", vec![Rule::Same("other")]);

    let result = validate(&rules, params);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().find(&["same"]).unwrap(),
               &Value::Boolean(true));
}

#[test]
fn test_same_invalid_boolean() {
    let mut params = Map::new();
    params.assign("same", Value::Boolean(true)).ok();
    params.assign("other", Value::Boolean(false)).ok();

    let mut rules = BTreeMap::new();
    rules.insert("same", vec![Rule::Same("other")]);

    let result = validate(&rules, params);

    assert!(result.is_err());
    assert_eq!(*result.unwrap_err().get("same").unwrap(),
               vec!["The same field must match the other field.".to_owned()]);
}

#[test]
fn test_same_valid_blank() {
    let params = Map::new();

    let mut rules = BTreeMap::new();
    rules.insert("same", vec![Rule::Same("other")]);

    let result = validate(&rules, params);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().find(&["same"]), None);
}

#[test]
fn test_same_invalid_null() {
    let mut params = Map::new();
    params.assign("same", Value::Null).ok();

    let mut rules = BTreeMap::new();
    rules.insert("same", vec![Rule::Same("other")]);

    let result = validate(&rules, params);

    assert!(result.is_err());
    assert_eq!(*result.unwrap_err().get("same").unwrap(),
               vec!["The same field must match the other field.".to_owned()]);
}

#[test]
fn test_same_valid_nested() {
    let mut test = Map::new();
    test.assign("same", Value::String("foo".to_owned())).ok();
    test.assign("other", Value::String("foo".to_owned())).ok();
    let mut params = Map::new();
    params.assign("test", Value::Map(test)).ok();

    let mut rules = BTreeMap::new();
    rules.insert("test.same", vec![Rule::Same("test.other")]);

    let result = validate(&rules, params);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().find(&["test", "same"]).unwrap(),
               &Value::String("foo".to_owned()));
}
