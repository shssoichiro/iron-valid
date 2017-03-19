extern crate iron_valid;
extern crate params;

use iron_valid::{Rule, validate};
use params::{Map, Value};
use std::collections::BTreeMap;

#[test]
fn test_present_valid_boolean_true() {
    let mut params = Map::new();
    params.assign("present", Value::Boolean(true)).ok();

    let mut rules = BTreeMap::new();
    rules.insert("present", vec![Rule::Present]);

    let result = validate(&rules, params);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().find(&["present"]).unwrap(),
               &Value::Boolean(true));
}

#[test]
fn test_present_valid_boolean_false() {
    let mut params = Map::new();
    params.assign("present", Value::Boolean(false)).ok();

    let mut rules = BTreeMap::new();
    rules.insert("present", vec![Rule::Present]);

    let result = validate(&rules, params);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().find(&["present"]).unwrap(),
               &Value::Boolean(false));
}

#[test]
fn test_present_valid_string() {
    let mut params = Map::new();
    params.assign("present", Value::String("true".to_owned())).ok();

    let mut rules = BTreeMap::new();
    rules.insert("present", vec![Rule::Present]);

    let result = validate(&rules, params);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().find(&["present"]).unwrap(),
               &Value::String("true".to_owned()));
}

#[test]
fn test_present_valid_empty_string() {
    let mut params = Map::new();
    params.assign("present", Value::String("".to_owned())).ok();

    let mut rules = BTreeMap::new();
    rules.insert("present", vec![Rule::Present]);

    let result = validate(&rules, params);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().find(&["present"]).unwrap(),
               &Value::String("".to_owned()));
}

#[test]
fn test_present_valid_numeric() {
    let mut params = Map::new();
    params.assign("present", Value::U64(1)).ok();

    let mut rules = BTreeMap::new();
    rules.insert("present", vec![Rule::Present]);

    let result = validate(&rules, params);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().find(&["present"]).unwrap(), &Value::U64(1));
}

#[test]
fn test_present_invalid_blank() {
    let params = Map::new();

    let mut rules = BTreeMap::new();
    rules.insert("present", vec![Rule::Present]);

    let result = validate(&rules, params);

    assert!(result.is_err());
    assert_eq!(*result.unwrap_err().get("present").unwrap(),
               vec!["The present field must be present.".to_owned()]);
}

#[test]
fn test_present_invalid_null() {
    let mut params = Map::new();
    params.assign("present", Value::Null).ok();

    let mut rules = BTreeMap::new();
    rules.insert("present", vec![Rule::Present]);

    let result = validate(&rules, params);

    assert!(result.is_err());
    assert_eq!(*result.unwrap_err().get("present").unwrap(),
               vec!["The present field must be present.".to_owned()]);
}

#[test]
fn test_present_valid_nested() {
    let mut test = Map::new();
    test.assign("present", Value::Boolean(true)).ok();
    let mut params = Map::new();
    params.assign("test", Value::Map(test)).ok();

    let mut rules = BTreeMap::new();
    rules.insert("test.present", vec![Rule::Present]);

    let result = validate(&rules, params);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().find(&["test", "present"]).unwrap(),
               &Value::Boolean(true));
}
