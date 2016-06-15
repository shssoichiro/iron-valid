extern crate iron_valid;
extern crate params;

use iron_valid::{Rule, validate};
use params::{Map, Value};
use std::collections::BTreeMap;

#[test]
fn test_required_with_valid() {
    let mut params = Map::new();
    params.assign("required", Value::String("true".to_owned())).ok();
    params.assign("other", Value::Boolean(true)).ok();

    let mut rules = BTreeMap::new();
    rules.insert("required", vec![Rule::RequiredWith(vec!["other"])]);

    let result = validate(rules, params);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().find(&["required"]).unwrap(),
               &Value::String("true".to_owned()));
}

#[test]
fn test_required_with_valid_one_of_two() {
    let mut params = Map::new();
    params.assign("required", Value::String("true".to_owned())).ok();
    params.assign("other", Value::Boolean(true)).ok();

    let mut rules = BTreeMap::new();
    rules.insert("required",
                 vec![Rule::RequiredWith(vec!["other", "another"])]);

    let result = validate(rules, params);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().find(&["required"]).unwrap(),
               &Value::String("true".to_owned()));
}

#[test]
fn test_required_with_valid_two_of_two() {
    let mut params = Map::new();
    params.assign("required", Value::String("true".to_owned())).ok();
    params.assign("other", Value::Boolean(true)).ok();
    params.assign("another", Value::Boolean(true)).ok();

    let mut rules = BTreeMap::new();
    rules.insert("required",
                 vec![Rule::RequiredWith(vec!["other", "another"])]);

    let result = validate(rules, params);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().find(&["required"]).unwrap(),
               &Value::String("true".to_owned()));
}

#[test]
fn test_required_with_invalid_one_of_two_blank() {
    let mut params = Map::new();
    params.assign("required", Value::String("".to_owned())).ok();
    params.assign("other", Value::Boolean(true)).ok();

    let mut rules = BTreeMap::new();
    rules.insert("required",
                 vec![Rule::RequiredWith(vec!["other", "another"])]);

    let result = validate(rules, params);

    assert!(result.is_err());
    assert_eq!(*result.unwrap_err().get("required").unwrap(),
               vec!["The required field is required.".to_owned()]);
}

#[test]
fn test_required_with_invalid_two_of_two_blank() {
    let mut params = Map::new();
    params.assign("required", Value::String("".to_owned())).ok();
    params.assign("other", Value::Boolean(true)).ok();
    params.assign("another", Value::Boolean(true)).ok();

    let mut rules = BTreeMap::new();
    rules.insert("required",
                 vec![Rule::RequiredWith(vec!["other", "another"])]);

    let result = validate(rules, params);

    assert!(result.is_err());
    assert_eq!(*result.unwrap_err().get("required").unwrap(),
               vec!["The required field is required.".to_owned()]);
}

#[test]
fn test_required_with_invalid() {
    let mut params = Map::new();
    params.assign("required", Value::String("".to_owned())).ok();
    params.assign("other", Value::Boolean(true)).ok();

    let mut rules = BTreeMap::new();
    rules.insert("required", vec![Rule::RequiredWith(vec!["other"])]);

    let result = validate(rules, params);

    assert!(result.is_err());
    assert_eq!(*result.unwrap_err().get("required").unwrap(),
               vec!["The required field is required.".to_owned()]);
}

#[test]
fn test_required_with_valid_not_match() {
    let mut params = Map::new();
    params.assign("required", Value::String("".to_owned())).ok();
    params.assign("other", Value::String("".to_owned())).ok();

    let mut rules = BTreeMap::new();
    rules.insert("required", vec![Rule::RequiredWith(vec!["other"])]);

    let result = validate(rules, params);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().find(&["required"]).unwrap(),
               &Value::String("".to_owned()));
}

#[test]
fn test_required_with_valid_other_blank() {
    let mut params = Map::new();
    params.assign("required", Value::String("".to_owned())).ok();

    let mut rules = BTreeMap::new();
    rules.insert("required", vec![Rule::RequiredWith(vec!["other"])]);

    let result = validate(rules, params);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().find(&["required"]).unwrap(),
               &Value::String("".to_owned()));
}

#[test]
fn test_required_with_invalid_blank() {
    let mut params = Map::new();
    params.assign("other", Value::Boolean(true)).ok();

    let mut rules = BTreeMap::new();
    rules.insert("required", vec![Rule::RequiredWith(vec!["other"])]);

    let result = validate(rules, params);

    assert!(result.is_err());
    assert_eq!(*result.unwrap_err().get("required").unwrap(),
               vec!["The required field is required.".to_owned()]);
}

#[test]
fn test_required_with_invalid_null() {
    let mut params = Map::new();
    params.assign("required", Value::Null).ok();
    params.assign("other", Value::Boolean(true)).ok();

    let mut rules = BTreeMap::new();
    rules.insert("required", vec![Rule::RequiredWith(vec!["other"])]);

    let result = validate(rules, params);

    assert!(result.is_err());
    assert_eq!(*result.unwrap_err().get("required").unwrap(),
               vec!["The required field is required.".to_owned()]);
}
