extern crate iron_valid;
extern crate params;

use iron_valid::{Rule, validate};
use params::{Map, Value};
use std::collections::BTreeMap;

#[test]
fn test_active_url_valid() {
    let mut params = Map::new();
    params.assign("active_url", Value::String("google.com".to_owned())).ok();

    let mut rules = BTreeMap::new();
    rules.insert("active_url", vec![Rule::ActiveUrl]);

    let result = validate(rules, params);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().find(&["active_url"]).unwrap(),
               &Value::String("google.com".to_owned()));
}

#[test]
fn test_active_url_invalid_domain() {
    let mut params = Map::new();
    params.assign("active_url",
                Value::String("vn4vau9n42n92f342j3298juinnu.com".to_owned()))
        .ok();

    let mut rules = BTreeMap::new();
    rules.insert("active_url", vec![Rule::ActiveUrl]);

    let result = validate(rules, params);

    assert!(result.is_err());
    assert_eq!(*result.unwrap_err().get("active_url").unwrap(),
               vec!["The active url field must contain a valid, active domain name.".to_owned()]);
}

#[test]
fn test_active_url_invalid_string() {
    let mut params = Map::new();
    params.assign("active_url", Value::String("foobar".to_owned())).ok();

    let mut rules = BTreeMap::new();
    rules.insert("active_url", vec![Rule::ActiveUrl]);

    let result = validate(rules, params);

    assert!(result.is_err());
    assert_eq!(*result.unwrap_err().get("active_url").unwrap(),
               vec!["The active url field must contain a valid, active domain name.".to_owned()]);
}

#[test]
fn test_active_url_invalid_numeric() {
    let mut params = Map::new();
    params.assign("active_url", Value::U64(42)).ok();

    let mut rules = BTreeMap::new();
    rules.insert("active_url", vec![Rule::ActiveUrl]);

    let result = validate(rules, params);

    assert!(result.is_err());
    assert_eq!(*result.unwrap_err().get("active_url").unwrap(),
               vec!["The active url field must contain a valid, active domain name.".to_owned()]);
}

#[test]
fn test_active_url_invalid_float() {
    let mut params = Map::new();
    params.assign("active_url", Value::F64(4.2)).ok();

    let mut rules = BTreeMap::new();
    rules.insert("active_url", vec![Rule::ActiveUrl]);

    let result = validate(rules, params);

    assert!(result.is_err());
    assert_eq!(*result.unwrap_err().get("active_url").unwrap(),
               vec!["The active url field must contain a valid, active domain name.".to_owned()]);
}

#[test]
fn test_active_url_valid_empty() {
    let mut params = Map::new();
    params.assign("active_url", Value::String("".to_owned())).ok();

    let mut rules = BTreeMap::new();
    rules.insert("active_url", vec![Rule::ActiveUrl]);

    let result = validate(rules, params);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().find(&["active_url"]).unwrap(),
               &Value::String("".to_owned()));
}

#[test]
fn test_active_url_valid_blank() {
    let params = Map::new();

    let mut rules = BTreeMap::new();
    rules.insert("active_url", vec![Rule::ActiveUrl]);

    let result = validate(rules, params);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().find(&["active_url"]), None);
}

#[test]
fn test_active_url_invalid_null() {
    let mut params = Map::new();
    params.assign("active_url", Value::Null).ok();

    let mut rules = BTreeMap::new();
    rules.insert("active_url", vec![Rule::ActiveUrl]);

    let result = validate(rules, params);

    assert!(result.is_err());
    assert_eq!(*result.unwrap_err().get("active_url").unwrap(),
               vec!["The active url field must contain a valid, active domain name.".to_owned()]);
}
