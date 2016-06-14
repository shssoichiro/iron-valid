extern crate iron_valid;
extern crate params;

use iron_valid::{Rule, validate};
use params::{Map, Value};
use std::collections::BTreeMap;

#[test]
fn test_required_if_valid() {
    let mut params = Map::new();
    params.assign("required", Value::String("true".to_owned())).ok();
    params.assign("other", Value::Boolean(true)).ok();

    let mut rules = BTreeMap::new();
    rules.insert("required",
                 vec![Rule::RequiredIf("other", Value::Boolean(true))]);

    let result = validate(rules, params);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().find(&["required"]).unwrap(),
               &Value::String("true".to_owned()));
}

#[test]
fn test_required_if_invalid() {
    let mut params = Map::new();
    params.assign("required", Value::String("".to_owned())).ok();
    params.assign("other", Value::Boolean(true)).ok();

    let mut rules = BTreeMap::new();
    rules.insert("required",
                 vec![Rule::RequiredIf("other", Value::Boolean(true))]);

    let result = validate(rules, params);

    assert!(result.is_err());
    assert_eq!(*result.unwrap_err().get("required").unwrap(),
               vec!["The required field is required.".to_owned()]);
}

#[test]
fn test_required_if_valid_not_match() {
    let mut params = Map::new();
    params.assign("required", Value::String("".to_owned())).ok();
    params.assign("other", Value::Boolean(false)).ok();

    let mut rules = BTreeMap::new();
    rules.insert("required",
                 vec![Rule::RequiredIf("other", Value::Boolean(true))]);

    let result = validate(rules, params);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().find(&["required"]).unwrap(),
               &Value::String("".to_owned()));
}

#[test]
fn test_required_if_valid_other_blank() {
    let mut params = Map::new();
    params.assign("required", Value::String("".to_owned())).ok();

    let mut rules = BTreeMap::new();
    rules.insert("required",
                 vec![Rule::RequiredIf("other", Value::Boolean(true))]);

    let result = validate(rules, params);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().find(&["required"]).unwrap(),
               &Value::String("".to_owned()));
}

#[test]
fn test_required_if_invalid_blank() {
    let mut params = Map::new();
    params.assign("other", Value::Boolean(true)).ok();

    let mut rules = BTreeMap::new();
    rules.insert("required",
                 vec![Rule::RequiredIf("other", Value::Boolean(true))]);

    let result = validate(rules, params);

    assert!(result.is_err());
    assert_eq!(*result.unwrap_err().get("required").unwrap(),
               vec!["The required field is required.".to_owned()]);
}

#[test]
fn test_required_if_invalid_null() {
    let mut params = Map::new();
    params.assign("required", Value::Null).ok();
    params.assign("other", Value::Boolean(true)).ok();

    let mut rules = BTreeMap::new();
    rules.insert("required",
                 vec![Rule::RequiredIf("other", Value::Boolean(true))]);

    let result = validate(rules, params);

    assert!(result.is_err());
    assert_eq!(*result.unwrap_err().get("required").unwrap(),
               vec!["The required field is required.".to_owned()]);
}
