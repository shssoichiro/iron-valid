extern crate iron_valid;
extern crate params;

use iron_valid::{Rule, validate};
use params::{Map, Value};
use std::collections::BTreeMap;

#[test]
fn test_json_valid() {
    let mut params = Map::new();
    params.assign("json", Value::String("{\"foo\": \"bar\"}".to_owned())).ok();

    let mut rules = BTreeMap::new();
    rules.insert("json", vec![Rule::Json]);

    let result = validate(&rules, params);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().find(&["json"]).unwrap(),
               &Value::String("{\"foo\": \"bar\"}".to_owned()));
}

#[test]
fn test_json_invalid_string() {
    let mut params = Map::new();
    params.assign("json", Value::String("foobar".to_owned())).ok();

    let mut rules = BTreeMap::new();
    rules.insert("json", vec![Rule::Json]);

    let result = validate(&rules, params);

    assert!(result.is_err());
    assert_eq!(*result.unwrap_err().get("json").unwrap(),
               vec!["The json field must contain a valid JSON string.".to_owned()]);
}

#[test]
fn test_json_invalid_numeric() {
    let mut params = Map::new();
    params.assign("json", Value::U64(42)).ok();

    let mut rules = BTreeMap::new();
    rules.insert("json", vec![Rule::Json]);

    let result = validate(&rules, params);

    assert!(result.is_err());
    assert_eq!(*result.unwrap_err().get("json").unwrap(),
               vec!["The json field must contain a valid JSON string.".to_owned()]);
}

#[test]
fn test_json_valid_empty() {
    let mut params = Map::new();
    params.assign("json", Value::String("".to_owned())).ok();

    let mut rules = BTreeMap::new();
    rules.insert("json", vec![Rule::Json]);

    let result = validate(&rules, params);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().find(&["json"]).unwrap(),
               &Value::String("".to_owned()));
}

#[test]
fn test_json_valid_blank() {
    let params = Map::new();

    let mut rules = BTreeMap::new();
    rules.insert("json", vec![Rule::Json]);

    let result = validate(&rules, params);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().find(&["json"]), None);
}

#[test]
fn test_json_invalid_null() {
    let mut params = Map::new();
    params.assign("json", Value::Null).ok();

    let mut rules = BTreeMap::new();
    rules.insert("json", vec![Rule::Json]);

    let result = validate(&rules, params);

    assert!(result.is_err());
    assert_eq!(*result.unwrap_err().get("json").unwrap(),
               vec!["The json field must contain a valid JSON string.".to_owned()]);
}

#[test]
fn test_json_valid_nested() {
    let mut test = Map::new();
    test.assign("json", Value::String("{\"foo\": \"bar\"}".to_owned())).ok();
    let mut params = Map::new();
    params.assign("test", Value::Map(test)).ok();

    let mut rules = BTreeMap::new();
    rules.insert("test.json", vec![Rule::Json]);

    let result = validate(&rules, params);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().find(&["test", "json"]).unwrap(),
               &Value::String("{\"foo\": \"bar\"}".to_owned()));
}
