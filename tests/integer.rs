extern crate iron_valid;
extern crate params;

use iron_valid::{Rule, validate};
use params::{Map, Value};
use std::collections::BTreeMap;

#[test]
fn test_integer_valid_string() {
    let mut params = Map::new();
    params.assign("integer", Value::String("3".to_owned())).ok();

    let mut rules = BTreeMap::new();
    rules.insert("integer", vec![Rule::Integer]);

    let result = validate(&rules, params);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().find(&["integer"]).unwrap(), &Value::U64(3));
}

#[test]
fn test_integer_valid_empty_string() {
    let mut params = Map::new();
    params.assign("integer", Value::String("".to_owned())).ok();

    let mut rules = BTreeMap::new();
    rules.insert("integer", vec![Rule::Integer]);

    let result = validate(&rules, params);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().find(&["integer"]).unwrap(),
               &Value::String("".to_owned()));
}

#[test]
fn test_integer_invalid_string() {
    let mut params = Map::new();
    params.assign("integer", Value::String("foo".to_owned())).ok();

    let mut rules = BTreeMap::new();
    rules.insert("integer", vec![Rule::Integer]);

    let result = validate(&rules, params);

    assert!(result.is_err());
    assert_eq!(*result.unwrap_err().get("integer").unwrap(),
               vec!["The integer field must be an integer.".to_owned()]);
}

#[test]
fn test_integer_valid_u64() {
    let mut params = Map::new();
    params.assign("integer", Value::U64(3)).ok();

    let mut rules = BTreeMap::new();
    rules.insert("integer", vec![Rule::Integer]);

    let result = validate(&rules, params);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().find(&["integer"]).unwrap(), &Value::U64(3));
}

#[test]
fn test_integer_valid_i64() {
    let mut params = Map::new();
    params.assign("integer", Value::I64(-3)).ok();

    let mut rules = BTreeMap::new();
    rules.insert("integer", vec![Rule::Integer]);

    let result = validate(&rules, params);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().find(&["integer"]).unwrap(), &Value::I64(-3));
}

#[test]
fn test_integer_invalid_f64() {
    let mut params = Map::new();
    params.assign("integer", Value::F64(10.0)).ok();

    let mut rules = BTreeMap::new();
    rules.insert("integer", vec![Rule::Integer]);

    let result = validate(&rules, params);

    assert!(result.is_err());
    assert_eq!(*result.unwrap_err().get("integer").unwrap(),
               vec!["The integer field must be an integer.".to_owned()]);
}

#[test]
fn test_integer_invalid_array() {
    let mut params = Map::new();
    params.assign("integer",
                  Value::Array(vec![Value::U64(1), Value::U64(2), Value::U64(3)]))
        .ok();

    let mut rules = BTreeMap::new();
    rules.insert("integer", vec![Rule::Integer]);

    let result = validate(&rules, params);

    assert!(result.is_err());
    assert_eq!(*result.unwrap_err().get("integer").unwrap(),
               vec!["The integer field must be an integer.".to_owned()]);
}

#[test]
fn test_integer_invalid_object() {
    let mut params = Map::new();
    let mut items = Map::new();
    items.assign("foo", Value::U64(1)).ok();
    items.assign("bar", Value::U64(2)).ok();
    items.assign("baz", Value::U64(3)).ok();
    params.assign("integer", Value::Map(items.clone())).ok();

    let mut rules = BTreeMap::new();
    rules.insert("integer", vec![Rule::Integer]);

    let result = validate(&rules, params);

    assert!(result.is_err());
    assert_eq!(*result.unwrap_err().get("integer").unwrap(),
               vec!["The integer field must be an integer.".to_owned()]);
}

#[test]
fn test_integer_valid_blank() {
    let params = Map::new();

    let mut rules = BTreeMap::new();
    rules.insert("integer", vec![Rule::Integer]);

    let result = validate(&rules, params);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().find(&["integer"]), None);
}

#[test]
fn test_integer_invalid_null() {
    let mut params = Map::new();
    params.assign("integer", Value::Null).ok();

    let mut rules = BTreeMap::new();
    rules.insert("integer", vec![Rule::Integer]);

    let result = validate(&rules, params);

    assert!(result.is_err());
    assert_eq!(*result.unwrap_err().get("integer").unwrap(),
               vec!["The integer field must be an integer.".to_owned()]);
}

#[test]
fn test_integer_valid_nested() {
    let mut test = Map::new();
    test.assign("integer", Value::String("3".to_owned())).ok();
    let mut params = Map::new();
    params.assign("test", Value::Map(test)).ok();

    let mut rules = BTreeMap::new();
    rules.insert("test.integer", vec![Rule::Integer]);

    let result = validate(&rules, params);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().find(&["test", "integer"]).unwrap(),
               &Value::U64(3));
}
