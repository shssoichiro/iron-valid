extern crate iron_valid;
extern crate params;

use iron_valid::{Rule, validate};
use params::{Map, Value};
use std::collections::BTreeMap;

#[test]
fn test_email_valid() {
    let mut params = Map::new();
    params.assign("email", Value::String("foo@bar.com".to_owned())).ok();

    let mut rules = BTreeMap::new();
    rules.insert("email", vec![Rule::Email]);

    let result = validate(rules, params);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().find(&["email"]).unwrap(),
               &Value::String("foo@bar.com".to_owned()));
}

#[test]
fn test_email_invalid_string() {
    let mut params = Map::new();
    params.assign("email", Value::String("foobar.com".to_owned())).ok();

    let mut rules = BTreeMap::new();
    rules.insert("email", vec![Rule::Email]);

    let result = validate(rules, params);

    assert!(result.is_err());
    assert_eq!(*result.unwrap_err().get("email").unwrap(),
               vec!["The email field must contain a valid email address.".to_owned()]);
}

#[test]
fn test_email_invalid_numeric() {
    let mut params = Map::new();
    params.assign("email", Value::U64(42)).ok();

    let mut rules = BTreeMap::new();
    rules.insert("email", vec![Rule::Email]);

    let result = validate(rules, params);

    assert!(result.is_err());
    assert_eq!(*result.unwrap_err().get("email").unwrap(),
               vec!["The email field must contain a valid email address.".to_owned()]);
}

#[test]
fn test_email_valid_empty() {
    let mut params = Map::new();
    params.assign("email", Value::String("".to_owned())).ok();

    let mut rules = BTreeMap::new();
    rules.insert("email", vec![Rule::Email]);

    let result = validate(rules, params);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().find(&["email"]).unwrap(),
               &Value::String("".to_owned()));
}

#[test]
fn test_email_valid_blank() {
    let params = Map::new();

    let mut rules = BTreeMap::new();
    rules.insert("email", vec![Rule::Email]);

    let result = validate(rules, params);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().find(&["email"]), None);
}

#[test]
fn test_email_valid_null() {
    let mut params = Map::new();
    params.assign("email", Value::Null).ok();

    let mut rules = BTreeMap::new();
    rules.insert("email", vec![Rule::Email]);

    let result = validate(rules, params);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().find(&["email"]).unwrap(), &Value::Null);
}
