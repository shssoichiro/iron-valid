extern crate iron_valid;
extern crate params;

use iron_valid::{Rule, validate};
use params::{Map, Value};
use std::collections::BTreeMap;

#[test]
fn test_alpha_valid_lowercase() {
    let mut params = Map::new();
    params.assign("alpha", Value::String("foobarbaz".to_owned())).ok();

    let mut rules = BTreeMap::new();
    rules.insert("alpha", vec![Rule::Alpha]);

    let result = validate(rules, params);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().find(&["alpha"]).unwrap(),
               &Value::String("foobarbaz".to_owned()));
}

#[test]
fn test_alpha_valid_uppercase() {
    let mut params = Map::new();
    params.assign("alpha", Value::String("FOOBARBAZ".to_owned())).ok();

    let mut rules = BTreeMap::new();
    rules.insert("alpha", vec![Rule::Alpha]);

    let result = validate(rules, params);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().find(&["alpha"]).unwrap(),
               &Value::String("FOOBARBAZ".to_owned()));
}


#[test]
fn test_alpha_valid_mixed_case() {
    let mut params = Map::new();
    params.assign("alpha", Value::String("FooBarBaz".to_owned())).ok();

    let mut rules = BTreeMap::new();
    rules.insert("alpha", vec![Rule::Alpha]);

    let result = validate(rules, params);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().find(&["alpha"]).unwrap(),
               &Value::String("FooBarBaz".to_owned()));
}

#[test]
fn test_alpha_invalid_numeric_string() {
    let mut params = Map::new();
    params.assign("alpha", Value::String("f00barbaz".to_owned())).ok();

    let mut rules = BTreeMap::new();
    rules.insert("alpha", vec![Rule::Alpha]);

    let result = validate(rules, params);

    assert!(result.is_err());
    assert_eq!(*result.unwrap_err().get("alpha").unwrap(),
               vec!["The alpha field may only contain alphabetic characters.".to_owned()]);
}

#[test]
fn test_alpha_invalid_whitespace() {
    let mut params = Map::new();
    params.assign("alpha", Value::String("foo bar baz".to_owned())).ok();

    let mut rules = BTreeMap::new();
    rules.insert("alpha", vec![Rule::Alpha]);

    let result = validate(rules, params);

    assert!(result.is_err());
    assert_eq!(*result.unwrap_err().get("alpha").unwrap(),
               vec!["The alpha field may only contain alphabetic characters.".to_owned()]);
}

#[test]
fn test_alpha_invalid_underscore() {
    let mut params = Map::new();
    params.assign("alpha", Value::String("foo_bar_baz".to_owned())).ok();

    let mut rules = BTreeMap::new();
    rules.insert("alpha", vec![Rule::Alpha]);

    let result = validate(rules, params);

    assert!(result.is_err());
    assert_eq!(*result.unwrap_err().get("alpha").unwrap(),
               vec!["The alpha field may only contain alphabetic characters.".to_owned()]);
}

#[test]
fn test_alpha_invalid_dash() {
    let mut params = Map::new();
    params.assign("alpha", Value::String("foo-bar-baz".to_owned())).ok();

    let mut rules = BTreeMap::new();
    rules.insert("alpha", vec![Rule::Alpha]);

    let result = validate(rules, params);

    assert!(result.is_err());
    assert_eq!(*result.unwrap_err().get("alpha").unwrap(),
               vec!["The alpha field may only contain alphabetic characters.".to_owned()]);
}

#[test]
fn test_alpha_invalid_numeric() {
    let mut params = Map::new();
    params.assign("alpha", Value::U64(42)).ok();

    let mut rules = BTreeMap::new();
    rules.insert("alpha", vec![Rule::Alpha]);

    let result = validate(rules, params);

    assert!(result.is_err());
    assert_eq!(*result.unwrap_err().get("alpha").unwrap(),
               vec!["The alpha field may only contain alphabetic characters.".to_owned()]);
}

#[test]
fn test_alpha_valid_empty() {
    let mut params = Map::new();
    params.assign("alpha", Value::String("".to_owned())).ok();

    let mut rules = BTreeMap::new();
    rules.insert("alpha", vec![Rule::Alpha]);

    let result = validate(rules, params);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().find(&["alpha"]).unwrap(),
               &Value::String("".to_owned()));
}

#[test]
fn test_alpha_valid_blank() {
    let params = Map::new();

    let mut rules = BTreeMap::new();
    rules.insert("alpha", vec![Rule::Alpha]);

    let result = validate(rules, params);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().find(&["alpha"]), None);
}

#[test]
fn test_alpha_valid_null() {
    let mut params = Map::new();
    params.assign("alpha", Value::Null).ok();

    let mut rules = BTreeMap::new();
    rules.insert("alpha", vec![Rule::Alpha]);

    let result = validate(rules, params);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().find(&["alpha"]).unwrap(), &Value::Null);
}
