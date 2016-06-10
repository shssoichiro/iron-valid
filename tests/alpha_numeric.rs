extern crate iron_valid;
extern crate params;

use iron_valid::{Rule, validate};
use params::{Map, Value};
use std::collections::BTreeMap;

#[test]
fn test_alpha_numeric_valid_lowercase() {
    let mut params = Map::new();
    params.assign("alpha_numeric", Value::String("foobarbaz".to_owned())).ok();

    let mut rules = BTreeMap::new();
    rules.insert("alpha_numeric", vec![Rule::AlphaNumeric]);

    let result = validate(rules, params);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().find(&["alpha_numeric"]).unwrap(),
               &Value::String("foobarbaz".to_owned()));
}

#[test]
fn test_alpha_numeric_valid_uppercase() {
    let mut params = Map::new();
    params.assign("alpha_numeric", Value::String("FOOBARBAZ".to_owned())).ok();

    let mut rules = BTreeMap::new();
    rules.insert("alpha_numeric", vec![Rule::AlphaNumeric]);

    let result = validate(rules, params);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().find(&["alpha_numeric"]).unwrap(),
               &Value::String("FOOBARBAZ".to_owned()));
}


#[test]
fn test_alpha_numeric_valid_mixed_case() {
    let mut params = Map::new();
    params.assign("alpha_numeric", Value::String("FooBarBaz".to_owned())).ok();

    let mut rules = BTreeMap::new();
    rules.insert("alpha_numeric", vec![Rule::AlphaNumeric]);

    let result = validate(rules, params);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().find(&["alpha_numeric"]).unwrap(),
               &Value::String("FooBarBaz".to_owned()));
}

#[test]
fn test_alpha_numeric_valid_numeric_string() {
    let mut params = Map::new();
    params.assign("alpha_numeric", Value::String("f00barbaz".to_owned())).ok();

    let mut rules = BTreeMap::new();
    rules.insert("alpha_numeric", vec![Rule::AlphaNumeric]);

    let result = validate(rules, params);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().find(&["alpha_numeric"]).unwrap(),
               &Value::String("f00barbaz".to_owned()));
}

#[test]
fn test_alpha_numeric_invalid_whitespace() {
    let mut params = Map::new();
    params.assign("alpha_numeric", Value::String("foo bar baz".to_owned())).ok();

    let mut rules = BTreeMap::new();
    rules.insert("alpha_numeric", vec![Rule::AlphaNumeric]);

    let result = validate(rules, params);

    assert!(result.is_err());
    assert_eq!(*result.unwrap_err().get("alpha_numeric").unwrap(),
               vec!["The alpha numeric field may only contain alphanumeric characters."
                        .to_owned()]);
}

#[test]
fn test_alpha_numeric_invalid_underscore() {
    let mut params = Map::new();
    params.assign("alpha_numeric", Value::String("foo_bar_baz".to_owned())).ok();

    let mut rules = BTreeMap::new();
    rules.insert("alpha_numeric", vec![Rule::AlphaNumeric]);

    let result = validate(rules, params);

    assert!(result.is_err());
    assert_eq!(*result.unwrap_err().get("alpha_numeric").unwrap(),
               vec!["The alpha numeric field may only contain alphanumeric characters."
                        .to_owned()]);
}

#[test]
fn test_alpha_numeric_invalid_dash() {
    let mut params = Map::new();
    params.assign("alpha_numeric", Value::String("foo-bar-baz".to_owned())).ok();

    let mut rules = BTreeMap::new();
    rules.insert("alpha_numeric", vec![Rule::AlphaNumeric]);

    let result = validate(rules, params);

    assert!(result.is_err());
    assert_eq!(*result.unwrap_err().get("alpha_numeric").unwrap(),
               vec!["The alpha numeric field may only contain alphanumeric characters."
                        .to_owned()]);
}

#[test]
fn test_alpha_numeric_valid_numeric() {
    let mut params = Map::new();
    params.assign("alpha_numeric", Value::U64(42)).ok();

    let mut rules = BTreeMap::new();
    rules.insert("alpha_numeric", vec![Rule::AlphaNumeric]);

    let result = validate(rules, params);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().find(&["alpha_numeric"]).unwrap(),
               &Value::String("42".to_owned()));
}

#[test]
fn test_alpha_numeric_invalid_negative() {
    let mut params = Map::new();
    params.assign("alpha_numeric", Value::I64(-42)).ok();

    let mut rules = BTreeMap::new();
    rules.insert("alpha_numeric", vec![Rule::AlphaNumeric]);

    let result = validate(rules, params);

    assert!(result.is_err());
    assert_eq!(*result.unwrap_err().get("alpha_numeric").unwrap(),
               vec!["The alpha numeric field may only contain alphanumeric characters."
                        .to_owned()]);
}

#[test]
fn test_alpha_numeric_invalid_float() {
    let mut params = Map::new();
    params.assign("alpha_numeric", Value::F64(42.1)).ok();

    let mut rules = BTreeMap::new();
    rules.insert("alpha_numeric", vec![Rule::AlphaNumeric]);

    let result = validate(rules, params);

    assert!(result.is_err());
    assert_eq!(*result.unwrap_err().get("alpha_numeric").unwrap(),
               vec!["The alpha numeric field may only contain alphanumeric characters."
                        .to_owned()]);
}

#[test]
fn test_alpha_numeric_valid_empty() {
    let mut params = Map::new();
    params.assign("alpha_numeric", Value::String("".to_owned())).ok();

    let mut rules = BTreeMap::new();
    rules.insert("alpha_numeric", vec![Rule::AlphaNumeric]);

    let result = validate(rules, params);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().find(&["alpha_numeric"]).unwrap(),
               &Value::String("".to_owned()));
}

#[test]
fn test_alpha_numeric_valid_blank() {
    let params = Map::new();

    let mut rules = BTreeMap::new();
    rules.insert("alpha_numeric", vec![Rule::AlphaNumeric]);

    let result = validate(rules, params);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().find(&["alpha_numeric"]), None);
}

#[test]
fn test_alpha_numeric_invalid_null() {
    let mut params = Map::new();
    params.assign("alpha_numeric", Value::Null).ok();

    let mut rules = BTreeMap::new();
    rules.insert("alpha_numeric", vec![Rule::AlphaNumeric]);

    let result = validate(rules, params);

    assert!(result.is_err());
    assert_eq!(*result.unwrap_err().get("alpha_numeric").unwrap(),
               vec!["The alpha numeric field may only contain alphanumeric characters."
                        .to_owned()]);
}
