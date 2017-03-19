extern crate iron_valid;
extern crate params;

use iron_valid::{Rule, validate};
use params::{Map, Value};
use std::collections::BTreeMap;

#[test]
fn test_confirmed_valid_string() {
    let mut params = Map::new();
    params.assign("confirmed", Value::String("foo".to_owned())).ok();
    params.assign("confirmed_confirmation", Value::String("foo".to_owned())).ok();

    let mut rules = BTreeMap::new();
    rules.insert("confirmed", vec![Rule::Confirmed]);

    let result = validate(&rules, params);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().find(&["confirmed"]).unwrap(),
               &Value::String("foo".to_owned()));
}

#[test]
fn test_confirmed_valid_empty_string() {
    let mut params = Map::new();
    params.assign("confirmed", Value::String("".to_owned())).ok();
    params.assign("confirmed_confirmation", Value::String("".to_owned())).ok();

    let mut rules = BTreeMap::new();
    rules.insert("confirmed", vec![Rule::Confirmed]);

    let result = validate(&rules, params);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().find(&["confirmed"]).unwrap(),
               &Value::String("".to_owned()));
}

#[test]
fn test_confirmed_invalid_string() {
    let mut params = Map::new();
    params.assign("confirmed", Value::String("foo".to_owned())).ok();
    params.assign("confirmed_confirmation", Value::String("bar".to_owned())).ok();

    let mut rules = BTreeMap::new();
    rules.insert("confirmed", vec![Rule::Confirmed]);

    let result = validate(&rules, params);

    assert!(result.is_err());
    assert_eq!(*result.unwrap_err().get("confirmed").unwrap(),
               vec!["The confirmed field must match the confirmed confirmation.".to_owned()]);
}

#[test]
fn test_confirmed_valid_u64() {
    let mut params = Map::new();
    params.assign("confirmed", Value::U64(42)).ok();
    params.assign("confirmed_confirmation", Value::U64(42)).ok();

    let mut rules = BTreeMap::new();
    rules.insert("confirmed", vec![Rule::Confirmed]);

    let result = validate(&rules, params);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().find(&["confirmed"]).unwrap(),
               &Value::U64(42));
}

#[test]
fn test_confirmed_invalid_u64() {
    let mut params = Map::new();
    params.assign("confirmed", Value::U64(42)).ok();
    params.assign("confirmed_confirmation", Value::U64(41)).ok();

    let mut rules = BTreeMap::new();
    rules.insert("confirmed", vec![Rule::Confirmed]);

    let result = validate(&rules, params);

    assert!(result.is_err());
    assert_eq!(*result.unwrap_err().get("confirmed").unwrap(),
               vec!["The confirmed field must match the confirmed confirmation.".to_owned()]);
}

#[test]
fn test_confirmed_valid_i64() {
    let mut params = Map::new();
    params.assign("confirmed", Value::I64(42)).ok();
    params.assign("confirmed_confirmation", Value::I64(42)).ok();

    let mut rules = BTreeMap::new();
    rules.insert("confirmed", vec![Rule::Confirmed]);

    let result = validate(&rules, params);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().find(&["confirmed"]).unwrap(),
               &Value::I64(42));
}

#[test]
fn test_confirmed_invalid_i64() {
    let mut params = Map::new();
    params.assign("confirmed", Value::I64(42)).ok();
    params.assign("confirmed_confirmation", Value::I64(41)).ok();

    let mut rules = BTreeMap::new();
    rules.insert("confirmed", vec![Rule::Confirmed]);

    let result = validate(&rules, params);

    assert!(result.is_err());
    assert_eq!(*result.unwrap_err().get("confirmed").unwrap(),
               vec!["The confirmed field must match the confirmed confirmation.".to_owned()]);
}

#[test]
fn test_confirmed_valid_f64() {
    let mut params = Map::new();
    params.assign("confirmed", Value::F64(42.0)).ok();
    params.assign("confirmed_confirmation", Value::F64(42.0)).ok();

    let mut rules = BTreeMap::new();
    rules.insert("confirmed", vec![Rule::Confirmed]);

    let result = validate(&rules, params);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().find(&["confirmed"]).unwrap(),
               &Value::F64(42.0));
}

#[test]
fn test_confirmed_invalid_f64() {
    let mut params = Map::new();
    params.assign("confirmed", Value::F64(42.0)).ok();
    params.assign("confirmed_confirmation", Value::F64(41.0)).ok();

    let mut rules = BTreeMap::new();
    rules.insert("confirmed", vec![Rule::Confirmed]);

    let result = validate(&rules, params);

    assert!(result.is_err());
    assert_eq!(*result.unwrap_err().get("confirmed").unwrap(),
               vec!["The confirmed field must match the confirmed confirmation.".to_owned()]);
}

#[test]
fn test_confirmed_valid_array() {
    let mut params = Map::new();
    params.assign("confirmed", Value::Array(vec![Value::F64(42.0)])).ok();
    params.assign("confirmed_confirmation",
                  Value::Array(vec![Value::F64(42.0)]))
        .ok();

    let mut rules = BTreeMap::new();
    rules.insert("confirmed", vec![Rule::Confirmed]);

    let result = validate(&rules, params);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().find(&["confirmed"]).unwrap(),
               &Value::Array(vec![Value::F64(42.0)]));
}

#[test]
fn test_confirmed_valid_empty_array() {
    let mut params = Map::new();
    params.assign("confirmed", Value::Array(vec![])).ok();
    params.assign("confirmed_confirmation", Value::Array(vec![])).ok();

    let mut rules = BTreeMap::new();
    rules.insert("confirmed", vec![Rule::Confirmed]);

    let result = validate(&rules, params);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().find(&["confirmed"]).unwrap(),
               &Value::Array(vec![]));
}

#[test]
fn test_confirmed_invalid_array() {
    let mut params = Map::new();
    params.assign("confirmed", Value::Array(vec![Value::F64(42.0)])).ok();
    params.assign("confirmed_confirmation",
                  Value::Array(vec![Value::F64(41.0)]))
        .ok();

    let mut rules = BTreeMap::new();
    rules.insert("confirmed", vec![Rule::Confirmed]);

    let result = validate(&rules, params);

    assert!(result.is_err());
    assert_eq!(*result.unwrap_err().get("confirmed").unwrap(),
               vec!["The confirmed field must match the confirmed confirmation.".to_owned()]);
}

#[test]
fn test_confirmed_valid_object() {
    let mut params = Map::new();
    let mut items = Map::new();
    items.assign("foo", Value::U64(1)).ok();
    params.assign("confirmed", Value::Map(items.clone())).ok();
    params.assign("confirmed_confirmation", Value::Map(items.clone())).ok();

    let mut rules = BTreeMap::new();
    rules.insert("confirmed", vec![Rule::Confirmed]);

    let result = validate(&rules, params);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().find(&["confirmed"]).unwrap(),
               &Value::Map(items));
}

#[test]
fn test_confirmed_valid_empty_object() {
    let mut params = Map::new();
    let items = Map::new();
    params.assign("confirmed", Value::Map(items.clone())).ok();
    params.assign("confirmed_confirmation", Value::Map(items.clone())).ok();

    let mut rules = BTreeMap::new();
    rules.insert("confirmed", vec![Rule::Confirmed]);

    let result = validate(&rules, params);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().find(&["confirmed"]).unwrap(),
               &Value::Map(items));
}

#[test]
fn test_confirmed_invalid_object() {
    let mut params = Map::new();
    let mut items = Map::new();
    let empty_items = items.clone();
    items.assign("foo", Value::U64(1)).ok();
    params.assign("confirmed", Value::Map(items.clone())).ok();
    params.assign("confirmed_confirmation", Value::Map(empty_items.clone())).ok();

    let mut rules = BTreeMap::new();
    rules.insert("confirmed", vec![Rule::Confirmed]);

    let result = validate(&rules, params);

    assert!(result.is_err());
    assert_eq!(*result.unwrap_err().get("confirmed").unwrap(),
               vec!["The confirmed field must match the confirmed confirmation.".to_owned()]);
}

#[test]
fn test_confirmed_valid_boolean() {
    let mut params = Map::new();
    params.assign("confirmed", Value::Boolean(true)).ok();
    params.assign("confirmed_confirmation", Value::Boolean(true)).ok();

    let mut rules = BTreeMap::new();
    rules.insert("confirmed", vec![Rule::Confirmed]);

    let result = validate(&rules, params);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().find(&["confirmed"]).unwrap(),
               &Value::Boolean(true));
}

#[test]
fn test_confirmed_invalid_boolean() {
    let mut params = Map::new();
    params.assign("confirmed", Value::Boolean(true)).ok();
    params.assign("confirmed_confirmation", Value::Boolean(false)).ok();

    let mut rules = BTreeMap::new();
    rules.insert("confirmed", vec![Rule::Confirmed]);

    let result = validate(&rules, params);

    assert!(result.is_err());
    assert_eq!(*result.unwrap_err().get("confirmed").unwrap(),
               vec!["The confirmed field must match the confirmed confirmation.".to_owned()]);
}

#[test]
fn test_confirmed_valid_blank() {
    let params = Map::new();

    let mut rules = BTreeMap::new();
    rules.insert("confirmed", vec![Rule::Confirmed]);

    let result = validate(&rules, params);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().find(&["confirmed"]), None);
}

#[test]
fn test_confirmed_invalid_null() {
    let mut params = Map::new();
    params.assign("confirmed", Value::Null).ok();

    let mut rules = BTreeMap::new();
    rules.insert("confirmed", vec![Rule::Confirmed]);

    let result = validate(&rules, params);

    assert!(result.is_err());
    assert_eq!(*result.unwrap_err().get("confirmed").unwrap(),
               vec!["The confirmed field must match the confirmed confirmation.".to_owned()]);
}

#[test]
fn test_confirmed_valid_nested() {
    let mut test = Map::new();
    test.assign("confirmed", Value::String("foo".to_owned())).ok();
    test.assign("confirmed_confirmation", Value::String("foo".to_owned())).ok();
    let mut params = Map::new();
    params.assign("test", Value::Map(test)).ok();

    let mut rules = BTreeMap::new();
    rules.insert("test.confirmed", vec![Rule::Confirmed]);

    let result = validate(&rules, params);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().find(&["test", "confirmed"]).unwrap(),
               &Value::String("foo".to_owned()));
}
