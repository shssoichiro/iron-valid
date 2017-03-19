extern crate iron_valid;
extern crate params;

use iron_valid::{Rule, validate};
use params::{Map, Value};
use std::collections::BTreeMap;

#[test]
fn test_accepted_yes_valid() {
    let mut params = Map::new();
    params.assign("accepted", Value::String("yes".to_owned())).ok();

    let mut rules = BTreeMap::new();
    rules.insert("accepted", vec![Rule::Accepted]);

    let result = validate(&rules, params);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().find(&["accepted"]).unwrap(),
               &Value::Boolean(true));
}

#[test]
fn test_accepted_on_valid() {
    let mut params = Map::new();
    params.assign("accepted", Value::String("on".to_owned())).ok();

    let mut rules = BTreeMap::new();
    rules.insert("accepted", vec![Rule::Accepted]);

    let result = validate(&rules, params);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().find(&["accepted"]).unwrap(),
               &Value::Boolean(true));
}

#[test]
fn test_accepted_true_string_valid() {
    let mut params = Map::new();
    params.assign("accepted", Value::String("true".to_owned())).ok();

    let mut rules = BTreeMap::new();
    rules.insert("accepted", vec![Rule::Accepted]);

    let result = validate(&rules, params);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().find(&["accepted"]).unwrap(),
               &Value::Boolean(true));
}

#[test]
fn test_accepted_true_boolean_valid() {
    let mut params = Map::new();
    params.assign("accepted", Value::Boolean(true)).ok();

    let mut rules = BTreeMap::new();
    rules.insert("accepted", vec![Rule::Accepted]);

    let result = validate(&rules, params);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().find(&["accepted"]).unwrap(),
               &Value::Boolean(true));
}

#[test]
fn test_accepted_1_string_valid() {
    let mut params = Map::new();
    params.assign("accepted", Value::String("1".to_owned())).ok();

    let mut rules = BTreeMap::new();
    rules.insert("accepted", vec![Rule::Accepted]);

    let result = validate(&rules, params);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().find(&["accepted"]).unwrap(),
               &Value::Boolean(true));
}

#[test]
fn test_accepted_1_i64_valid() {
    let mut params = Map::new();
    params.assign("accepted", Value::I64(1)).ok();

    let mut rules = BTreeMap::new();
    rules.insert("accepted", vec![Rule::Accepted]);

    let result = validate(&rules, params);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().find(&["accepted"]).unwrap(),
               &Value::Boolean(true));
}

#[test]
fn test_accepted_1_u64_valid() {
    let mut params = Map::new();
    params.assign("accepted", Value::U64(1)).ok();

    let mut rules = BTreeMap::new();
    rules.insert("accepted", vec![Rule::Accepted]);

    let result = validate(&rules, params);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().find(&["accepted"]).unwrap(),
               &Value::Boolean(true));
}

#[test]
fn test_accepted_0_u64_invalid() {
    let mut params = Map::new();
    params.assign("accepted", Value::U64(0)).ok();

    let mut rules = BTreeMap::new();
    rules.insert("accepted", vec![Rule::Accepted]);

    let result = validate(&rules, params);

    assert!(result.is_err());
    assert_eq!(*result.unwrap_err().get("accepted").unwrap(),
               vec!["The accepted must be accepted.".to_owned()]);
}

#[test]
fn test_accepted_0_i64_invalid() {
    let mut params = Map::new();
    params.assign("accepted", Value::I64(0)).ok();

    let mut rules = BTreeMap::new();
    rules.insert("accepted", vec![Rule::Accepted]);

    let result = validate(&rules, params);

    assert!(result.is_err());
    assert_eq!(*result.unwrap_err().get("accepted").unwrap(),
               vec!["The accepted must be accepted.".to_owned()]);
}

#[test]
fn test_accepted_1_f64_invalid() {
    let mut params = Map::new();
    params.assign("accepted", Value::F64(1.0)).ok();

    let mut rules = BTreeMap::new();
    rules.insert("accepted", vec![Rule::Accepted]);

    let result = validate(&rules, params);

    assert!(result.is_err());
    assert_eq!(*result.unwrap_err().get("accepted").unwrap(),
               vec!["The accepted must be accepted.".to_owned()]);
}

#[test]
fn test_accepted_false_boolean_invalid() {
    let mut params = Map::new();
    params.assign("accepted", Value::F64(1.0)).ok();

    let mut rules = BTreeMap::new();
    rules.insert("accepted", vec![Rule::Accepted]);

    let result = validate(&rules, params);

    assert!(result.is_err());
    assert_eq!(*result.unwrap_err().get("accepted").unwrap(),
               vec!["The accepted must be accepted.".to_owned()]);
}

#[test]
fn test_accepted_false_string_invalid() {
    let mut params = Map::new();
    params.assign("accepted", Value::F64(1.0)).ok();

    let mut rules = BTreeMap::new();
    rules.insert("accepted", vec![Rule::Accepted]);

    let result = validate(&rules, params);

    assert!(result.is_err());
    assert_eq!(*result.unwrap_err().get("accepted").unwrap(),
               vec!["The accepted must be accepted.".to_owned()]);
}

#[test]
fn test_accepted_empty_string_invalid() {
    let mut params = Map::new();
    params.assign("accepted", Value::String("".to_owned())).ok();

    let mut rules = BTreeMap::new();
    rules.insert("accepted", vec![Rule::Accepted]);

    let result = validate(&rules, params);

    assert!(result.is_err());
    assert_eq!(*result.unwrap_err().get("accepted").unwrap(),
               vec!["The accepted must be accepted.".to_owned()]);
}

#[test]
fn test_accepted_blank_invalid() {
    let params = Map::new();

    let mut rules = BTreeMap::new();
    rules.insert("accepted", vec![Rule::Accepted]);

    let result = validate(&rules, params);

    assert!(result.is_err());
    assert_eq!(*result.unwrap_err().get("accepted").unwrap(),
               vec!["The accepted must be accepted.".to_owned()]);
}

#[test]
fn test_accepted_null_invalid() {
    let mut params = Map::new();
    params.assign("accepted", Value::Null).ok();

    let mut rules = BTreeMap::new();
    rules.insert("accepted", vec![Rule::Accepted]);

    let result = validate(&rules, params);

    assert!(result.is_err());
    assert_eq!(*result.unwrap_err().get("accepted").unwrap(),
               vec!["The accepted must be accepted.".to_owned()]);
}

#[test]
fn test_accepted_nested_valid() {
    let mut test = Map::new();
    test.assign("accepted", Value::String("yes".to_owned())).ok();
    let mut params = Map::new();
    params.assign("test", Value::Map(test)).ok();

    let mut rules = BTreeMap::new();
    rules.insert("test.accepted", vec![Rule::Accepted]);

    let result = validate(&rules, params);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().find(&["test", "accepted"]).unwrap(),
               &Value::Boolean(true));
}
