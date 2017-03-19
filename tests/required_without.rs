extern crate iron_valid;
extern crate params;

use iron_valid::{Rule, validate};
use params::{Map, Value};
use std::collections::BTreeMap;

#[test]
fn test_required_without_valid() {
    let mut params = Map::new();
    params.assign("required", Value::String("true".to_owned())).ok();
    params.assign("other", Value::Boolean(true)).ok();

    let mut rules = BTreeMap::new();
    rules.insert("required", vec![Rule::RequiredWithout(vec!["other"])]);

    let result = validate(&rules, params);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().find(&["required"]).unwrap(),
               &Value::String("true".to_owned()));
}

#[test]
fn test_required_without_valid_one_of_two() {
    let mut params = Map::new();
    params.assign("required", Value::String("true".to_owned())).ok();
    params.assign("other", Value::Boolean(true)).ok();

    let mut rules = BTreeMap::new();
    rules.insert("required",
                 vec![Rule::RequiredWithout(vec!["other", "another"])]);

    let result = validate(&rules, params);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().find(&["required"]).unwrap(),
               &Value::String("true".to_owned()));
}

#[test]
fn test_required_without_valid_two_of_two() {
    let mut params = Map::new();
    params.assign("required", Value::String("true".to_owned())).ok();
    params.assign("other", Value::Boolean(true)).ok();
    params.assign("another", Value::Boolean(true)).ok();

    let mut rules = BTreeMap::new();
    rules.insert("required",
                 vec![Rule::RequiredWithout(vec!["other", "another"])]);

    let result = validate(&rules, params);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().find(&["required"]).unwrap(),
               &Value::String("true".to_owned()));
}

#[test]
fn test_required_without_invalid_one_of_two_blank() {
    let mut params = Map::new();
    params.assign("required", Value::String("".to_owned())).ok();
    params.assign("other", Value::Boolean(true)).ok();

    let mut rules = BTreeMap::new();
    rules.insert("required",
                 vec![Rule::RequiredWithout(vec!["other", "another"])]);

    let result = validate(&rules, params);

    assert!(result.is_err());
    assert_eq!(*result.unwrap_err().get("required").unwrap(),
               vec!["The required field is required.".to_owned()]);
}

#[test]
fn test_required_without_valid_two_of_two_blank() {
    let mut params = Map::new();
    params.assign("required", Value::String("".to_owned())).ok();
    params.assign("other", Value::Boolean(true)).ok();
    params.assign("another", Value::Boolean(true)).ok();

    let mut rules = BTreeMap::new();
    rules.insert("required",
                 vec![Rule::RequiredWithout(vec!["other", "another"])]);

    let result = validate(&rules, params);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().find(&["required"]).unwrap(),
               &Value::String("".to_owned()));
}

#[test]
fn test_required_without_valid_empty() {
    let mut params = Map::new();
    params.assign("required", Value::String("".to_owned())).ok();
    params.assign("other", Value::Boolean(true)).ok();

    let mut rules = BTreeMap::new();
    rules.insert("required", vec![Rule::RequiredWithout(vec!["other"])]);

    let result = validate(&rules, params);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().find(&["required"]).unwrap(),
               &Value::String("".to_owned()));
}

#[test]
fn test_required_without_invalid_other_blank() {
    let mut params = Map::new();
    params.assign("required", Value::String("".to_owned())).ok();

    let mut rules = BTreeMap::new();
    rules.insert("required", vec![Rule::RequiredWithout(vec!["other"])]);

    let result = validate(&rules, params);

    assert!(result.is_err());
    assert_eq!(*result.unwrap_err().get("required").unwrap(),
               vec!["The required field is required.".to_owned()]);
}

#[test]
fn test_required_without_valid_blank() {
    let mut params = Map::new();
    params.assign("other", Value::Boolean(true)).ok();

    let mut rules = BTreeMap::new();
    rules.insert("required", vec![Rule::RequiredWithout(vec!["other"])]);

    let result = validate(&rules, params);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().find(&["required"]), None);
}

#[test]
fn test_required_without_invalid_null() {
    let mut params = Map::new();
    params.assign("required", Value::Null).ok();

    let mut rules = BTreeMap::new();
    rules.insert("required", vec![Rule::RequiredWithout(vec!["other"])]);

    let result = validate(&rules, params);

    assert!(result.is_err());
    assert_eq!(*result.unwrap_err().get("required").unwrap(),
               vec!["The required field is required.".to_owned()]);
}

#[test]
fn test_required_without_valid_nested() {
    let mut test = Map::new();
    test.assign("required", Value::String("true".to_owned())).ok();
    test.assign("other", Value::Boolean(true)).ok();
    let mut params = Map::new();
    params.assign("test", Value::Map(test)).ok();

    let mut rules = BTreeMap::new();
    rules.insert("test.required",
                 vec![Rule::RequiredWithout(vec!["test.other"])]);

    let result = validate(&rules, params);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().find(&["test", "required"]).unwrap(),
               &Value::String("true".to_owned()));
}
