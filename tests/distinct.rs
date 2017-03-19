extern crate iron_valid;
extern crate params;

use iron_valid::{Rule, validate};
use params::{Map, Value};
use std::collections::BTreeMap;

#[test]
fn test_distinct_valid_array() {
    let mut params = Map::new();
    params.assign("distinct",
                  Value::Array(vec![Value::U64(1), Value::U64(2), Value::U64(3)]))
        .ok();

    let mut rules = BTreeMap::new();
    rules.insert("distinct", vec![Rule::Distinct]);

    let result = validate(&rules, params);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().find(&["distinct"]).unwrap(),
               &Value::Array(vec![Value::U64(1), Value::U64(2), Value::U64(3)]));
}

#[test]
fn test_distinct_invalid_array() {
    let mut params = Map::new();
    params.assign("distinct",
                  Value::Array(vec![Value::U64(1), Value::U64(2), Value::U64(1)]))
        .ok();

    let mut rules = BTreeMap::new();
    rules.insert("distinct", vec![Rule::Distinct]);

    let result = validate(&rules, params);

    assert!(result.is_err());
    assert_eq!(*result.unwrap_err().get("distinct").unwrap(),
               vec!["The distinct field must not contain any duplicate values.".to_owned()]);
}

#[test]
fn test_distinct_valid_string() {
    let mut params = Map::new();
    params.assign("distinct", Value::String("asdf".to_owned())).ok();

    let mut rules = BTreeMap::new();
    rules.insert("distinct", vec![Rule::Distinct]);

    let result = validate(&rules, params);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().find(&["distinct"]).unwrap(),
               &Value::String("asdf".to_owned()));
}

#[test]
fn test_distinct_valid_numeric() {
    let mut params = Map::new();
    params.assign("distinct", Value::U64(42)).ok();

    let mut rules = BTreeMap::new();
    rules.insert("distinct", vec![Rule::Distinct]);

    let result = validate(&rules, params);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().find(&["distinct"]).unwrap(),
               &Value::U64(42));
}

#[test]
fn test_distinct_valid_empty() {
    let mut params = Map::new();
    params.assign("distinct", Value::Array(vec![])).ok();

    let mut rules = BTreeMap::new();
    rules.insert("distinct", vec![Rule::Distinct]);

    let result = validate(&rules, params);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().find(&["distinct"]).unwrap(),
               &Value::Array(vec![]));
}

#[test]
fn test_distinct_valid_blank() {
    let params = Map::new();

    let mut rules = BTreeMap::new();
    rules.insert("distinct", vec![Rule::Distinct]);

    let result = validate(&rules, params);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().find(&["distinct"]), None);
}

#[test]
fn test_distinct_valid_null() {
    let mut params = Map::new();
    params.assign("distinct", Value::Null).ok();

    let mut rules = BTreeMap::new();
    rules.insert("distinct", vec![Rule::Distinct]);

    let result = validate(&rules, params);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().find(&["distinct"]).unwrap(), &Value::Null);
}

#[test]
fn test_distinct_valid_nested() {
    let mut test = Map::new();
    test.assign("distinct",
                Value::Array(vec![Value::U64(1), Value::U64(2), Value::U64(3)]))
        .ok();
    let mut params = Map::new();
    params.assign("test", Value::Map(test)).ok();

    let mut rules = BTreeMap::new();
    rules.insert("test.distinct", vec![Rule::Distinct]);

    let result = validate(&rules, params);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().find(&["test", "distinct"]).unwrap(),
               &Value::Array(vec![Value::U64(1), Value::U64(2), Value::U64(3)]));
}
