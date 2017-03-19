extern crate iron_valid;
extern crate params;

use iron_valid::{Rule, validate};
use params::{Map, Value};
use std::collections::BTreeMap;

#[test]
fn test_regex_valid_string() {
    let mut params = Map::new();
    params.assign("regex", Value::String("12345".to_owned())).ok();

    let mut rules = BTreeMap::new();
    rules.insert("regex", vec![Rule::Regex(r"^\d+$")]);

    let result = validate(&rules, params);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().find(&["regex"]).unwrap(),
               &Value::String("12345".to_owned()));
}

#[test]
fn test_regex_invalid_string_negative() {
    let mut params = Map::new();
    params.assign("regex", Value::String("-12345".to_owned())).ok();

    let mut rules = BTreeMap::new();
    rules.insert("regex", vec![Rule::Regex(r"^\d+$")]);

    let result = validate(&rules, params);

    assert!(result.is_err());
    assert_eq!(*result.unwrap_err().get("regex").unwrap(),
               vec!["The regex field must match the pattern \"^\\d+$\".".to_owned()]);
}

#[test]
fn test_regex_invalid_string_float() {
    let mut params = Map::new();
    params.assign("regex", Value::String("1234.56".to_owned())).ok();

    let mut rules = BTreeMap::new();
    rules.insert("regex", vec![Rule::Regex(r"^\d+$")]);

    let result = validate(&rules, params);

    assert!(result.is_err());
    assert_eq!(*result.unwrap_err().get("regex").unwrap(),
               vec!["The regex field must match the pattern \"^\\d+$\".".to_owned()]);
}

#[test]
fn test_regex_invalid_string_non_numeric() {
    let mut params = Map::new();
    params.assign("regex", Value::String("fooba".to_owned())).ok();

    let mut rules = BTreeMap::new();
    rules.insert("regex", vec![Rule::Regex(r"^\d+$")]);

    let result = validate(&rules, params);

    assert!(result.is_err());
    assert_eq!(*result.unwrap_err().get("regex").unwrap(),
               vec!["The regex field must match the pattern \"^\\d+$\".".to_owned()]);
}

#[test]
fn test_regex_invalid_empty_string() {
    let mut params = Map::new();
    params.assign("regex", Value::String("".to_owned())).ok();

    let mut rules = BTreeMap::new();
    rules.insert("regex", vec![Rule::Regex(r"^\d+$")]);

    let result = validate(&rules, params);

    assert!(result.is_err());
    assert_eq!(*result.unwrap_err().get("regex").unwrap(),
               vec!["The regex field must match the pattern \"^\\d+$\".".to_owned()]);
}

#[test]
fn test_regex_valid_u64() {
    let mut params = Map::new();
    params.assign("regex", Value::U64(12345)).ok();

    let mut rules = BTreeMap::new();
    rules.insert("regex", vec![Rule::Regex(r"^\d+$")]);

    let result = validate(&rules, params);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().find(&["regex"]).unwrap(),
               &Value::String("12345".to_owned()));
}

#[test]
fn test_regex_invalid_i64() {
    let mut params = Map::new();
    params.assign("regex", Value::I64(-12345)).ok();

    let mut rules = BTreeMap::new();
    rules.insert("regex", vec![Rule::Regex(r"^\d+$")]);

    let result = validate(&rules, params);

    assert!(result.is_err());
    assert_eq!(*result.unwrap_err().get("regex").unwrap(),
               vec!["The regex field must match the pattern \"^\\d+$\".".to_owned()]);
}

#[test]
fn test_regex_invalid_f64() {
    let mut params = Map::new();
    params.assign("regex", Value::F64(123456.7)).ok();

    let mut rules = BTreeMap::new();
    rules.insert("regex", vec![Rule::Regex(r"^\d+$")]);

    let result = validate(&rules, params);

    assert!(result.is_err());
    assert_eq!(*result.unwrap_err().get("regex").unwrap(),
               vec!["The regex field must match the pattern \"^\\d+$\".".to_owned()]);
}

#[test]
fn test_regex_valid_blank() {
    let params = Map::new();

    let mut rules = BTreeMap::new();
    rules.insert("regex", vec![Rule::Regex(r"^\d+$")]);

    let result = validate(&rules, params);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().find(&["regex"]), None);
}

#[test]
fn test_regex_invalid_null() {
    let mut params = Map::new();
    params.assign("regex", Value::Null).ok();

    let mut rules = BTreeMap::new();
    rules.insert("regex", vec![Rule::Regex(r"^\d+$")]);

    let result = validate(&rules, params);

    assert!(result.is_err());
    assert_eq!(*result.unwrap_err().get("regex").unwrap(),
               vec!["The regex field must match the pattern \"^\\d+$\".".to_owned()]);
}

#[test]
fn test_regex_valid_nested() {
    let mut test = Map::new();
    test.assign("regex", Value::String("12345".to_owned())).ok();
    let mut params = Map::new();
    params.assign("test", Value::Map(test)).ok();

    let mut rules = BTreeMap::new();
    rules.insert("test.regex", vec![Rule::Regex(r"^\d+$")]);

    let result = validate(&rules, params);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().find(&["test", "regex"]).unwrap(),
               &Value::String("12345".to_owned()));
}
