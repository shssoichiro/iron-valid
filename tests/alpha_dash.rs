extern crate iron_valid;
extern crate params;

use iron_valid::{Rule, validate};
use params::{Map, Value};
use std::collections::BTreeMap;

#[test]
fn test_alpha_dash_valid_lowercase() {
    let mut params = Map::new();
    params.assign("alpha_dash", Value::String("foobarbaz".to_owned())).ok();

    let mut rules = BTreeMap::new();
    rules.insert("alpha_dash", vec![Rule::AlphaDash]);

    let result = validate(rules, params);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().find(&["alpha_dash"]).unwrap(),
               &Value::String("foobarbaz".to_owned()));
}

#[test]
fn test_alpha_dash_valid_uppercase() {
    let mut params = Map::new();
    params.assign("alpha_dash", Value::String("FOOBARBAZ".to_owned())).ok();

    let mut rules = BTreeMap::new();
    rules.insert("alpha_dash", vec![Rule::AlphaDash]);

    let result = validate(rules, params);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().find(&["alpha_dash"]).unwrap(),
               &Value::String("FOOBARBAZ".to_owned()));
}


#[test]
fn test_alpha_dash_valid_mixed_case() {
    let mut params = Map::new();
    params.assign("alpha_dash", Value::String("FooBarBaz".to_owned())).ok();

    let mut rules = BTreeMap::new();
    rules.insert("alpha_dash", vec![Rule::AlphaDash]);

    let result = validate(rules, params);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().find(&["alpha_dash"]).unwrap(),
               &Value::String("FooBarBaz".to_owned()));
}

#[test]
fn test_alpha_dash_valid_numeric_string() {
    let mut params = Map::new();
    params.assign("alpha_dash", Value::String("f00barbaz".to_owned())).ok();

    let mut rules = BTreeMap::new();
    rules.insert("alpha_dash", vec![Rule::AlphaDash]);

    let result = validate(rules, params);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().find(&["alpha_dash"]).unwrap(),
               &Value::String("f00barbaz".to_owned()));
}

#[test]
fn test_alpha_dash_invalid_whitespace() {
    let mut params = Map::new();
    params.assign("alpha_dash", Value::String("foo bar baz".to_owned())).ok();

    let mut rules = BTreeMap::new();
    rules.insert("alpha_dash", vec![Rule::AlphaDash]);

    let result = validate(rules, params);

    assert!(result.is_err());
    assert_eq!(*result.unwrap_err().get("alpha_dash").unwrap(),
               vec!["The alpha dash field may only contain alphanumeric characters, dashes, and underscores."
                        .to_owned()]);
}

#[test]
fn test_alpha_dash_valid_underscore() {
    let mut params = Map::new();
    params.assign("alpha_dash", Value::String("foo_bar_baz".to_owned())).ok();

    let mut rules = BTreeMap::new();
    rules.insert("alpha_dash", vec![Rule::AlphaDash]);

    let result = validate(rules, params);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().find(&["alpha_dash"]).unwrap(),
               &Value::String("foo_bar_baz".to_owned()));
}

#[test]
fn test_alpha_dash_valid_dash() {
    let mut params = Map::new();
    params.assign("alpha_dash", Value::String("foo-bar-baz".to_owned())).ok();

    let mut rules = BTreeMap::new();
    rules.insert("alpha_dash", vec![Rule::AlphaDash]);

    let result = validate(rules, params);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().find(&["alpha_dash"]).unwrap(),
               &Value::String("foo-bar-baz".to_owned()));
}

#[test]
fn test_alpha_dash_valid_numeric() {
    let mut params = Map::new();
    params.assign("alpha_dash", Value::U64(42)).ok();

    let mut rules = BTreeMap::new();
    rules.insert("alpha_dash", vec![Rule::AlphaDash]);

    let result = validate(rules, params);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().find(&["alpha_dash"]).unwrap(),
               &Value::String("42".to_owned()));
}

#[test]
fn test_alpha_dash_valid_negative() {
    let mut params = Map::new();
    params.assign("alpha_dash", Value::I64(-42)).ok();

    let mut rules = BTreeMap::new();
    rules.insert("alpha_dash", vec![Rule::AlphaDash]);

    let result = validate(rules, params);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().find(&["alpha_dash"]).unwrap(),
               &Value::String("-42".to_owned()));
}

#[test]
fn test_alpha_dash_invalid_float() {
    let mut params = Map::new();
    params.assign("alpha_dash", Value::F64(42.1)).ok();

    let mut rules = BTreeMap::new();
    rules.insert("alpha_dash", vec![Rule::AlphaDash]);

    let result = validate(rules, params);

    assert!(result.is_err());
    assert_eq!(*result.unwrap_err().get("alpha_dash").unwrap(),
               vec!["The alpha dash field may only contain alphanumeric characters, dashes, and underscores."
                        .to_owned()]);
}

#[test]
fn test_alpha_dash_valid_empty() {
    let mut params = Map::new();
    params.assign("alpha_dash", Value::String("".to_owned())).ok();

    let mut rules = BTreeMap::new();
    rules.insert("alpha_dash", vec![Rule::AlphaDash]);

    let result = validate(rules, params);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().find(&["alpha_dash"]).unwrap(),
               &Value::String("".to_owned()));
}

#[test]
fn test_alpha_dash_valid_blank() {
    let params = Map::new();

    let mut rules = BTreeMap::new();
    rules.insert("alpha_dash", vec![Rule::AlphaDash]);

    let result = validate(rules, params);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().find(&["alpha_dash"]), None);
}

#[test]
fn test_alpha_dash_valid_null() {
    let mut params = Map::new();
    params.assign("alpha_dash", Value::Null).ok();

    let mut rules = BTreeMap::new();
    rules.insert("alpha_dash", vec![Rule::AlphaDash]);

    let result = validate(rules, params);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().find(&["alpha_dash"]).unwrap(), &Value::Null);
}
