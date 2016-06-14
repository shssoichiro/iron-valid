extern crate iron_valid;
extern crate params;

use iron_valid::{Rule, validate};
use params::{Map, Value};
use std::collections::BTreeMap;

#[test]
fn test_url_valid_prefixed() {
    let mut params = Map::new();
    params.assign("url", Value::String("http://foobar.com".to_owned())).ok();

    let mut rules = BTreeMap::new();
    rules.insert("url", vec![Rule::Url]);

    let result = validate(rules, params);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().find(&["url"]).unwrap(),
               &Value::String("http://foobar.com".to_owned()));
}

#[test]
fn test_url_valid_prefixed_secure() {
    let mut params = Map::new();
    params.assign("url", Value::String("https://foobar.com".to_owned())).ok();

    let mut rules = BTreeMap::new();
    rules.insert("url", vec![Rule::Url]);

    let result = validate(rules, params);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().find(&["url"]).unwrap(),
               &Value::String("https://foobar.com".to_owned()));
}

#[test]
fn test_url_invalid_no_prefix() {
    let mut params = Map::new();
    params.assign("url", Value::String("foobar.com".to_owned())).ok();

    let mut rules = BTreeMap::new();
    rules.insert("url", vec![Rule::Url]);

    let result = validate(rules, params);

    assert!(result.is_err());
    assert_eq!(*result.unwrap_err().get("url").unwrap(),
               vec!["The url field must contain a properly formatted URL.".to_owned()]);
}

#[test]
fn test_url_invalid_string() {
    let mut params = Map::new();
    params.assign("url", Value::String("foo@bar.com".to_owned())).ok();

    let mut rules = BTreeMap::new();
    rules.insert("url", vec![Rule::Url]);

    let result = validate(rules, params);

    assert!(result.is_err());
    assert_eq!(*result.unwrap_err().get("url").unwrap(),
               vec!["The url field must contain a properly formatted URL.".to_owned()]);
}

#[test]
fn test_url_invalid_numeric() {
    let mut params = Map::new();
    params.assign("url", Value::U64(42)).ok();

    let mut rules = BTreeMap::new();
    rules.insert("url", vec![Rule::Url]);

    let result = validate(rules, params);

    assert!(result.is_err());
    assert_eq!(*result.unwrap_err().get("url").unwrap(),
               vec!["The url field must contain a properly formatted URL.".to_owned()]);
}

#[test]
fn test_url_valid_empty() {
    let mut params = Map::new();
    params.assign("url", Value::String("".to_owned())).ok();

    let mut rules = BTreeMap::new();
    rules.insert("url", vec![Rule::Url]);

    let result = validate(rules, params);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().find(&["url"]).unwrap(),
               &Value::String("".to_owned()));
}

#[test]
fn test_url_valid_blank() {
    let params = Map::new();

    let mut rules = BTreeMap::new();
    rules.insert("url", vec![Rule::Url]);

    let result = validate(rules, params);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().find(&["url"]), None);
}

#[test]
fn test_url_invalid_null() {
    let mut params = Map::new();
    params.assign("url", Value::Null).ok();

    let mut rules = BTreeMap::new();
    rules.insert("url", vec![Rule::Url]);

    let result = validate(rules, params);

    assert!(result.is_err());
    assert_eq!(*result.unwrap_err().get("url").unwrap(),
               vec!["The url field must contain a properly formatted URL.".to_owned()]);
}
