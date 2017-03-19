extern crate iron_valid;
extern crate params;

use iron_valid::{Rule, validate};
use params::{Map, Value};
use std::collections::BTreeMap;

#[test]
fn test_digits_valid_string() {
    let mut params = Map::new();
    params.assign("digits", Value::String("12345".to_owned())).ok();

    let mut rules = BTreeMap::new();
    rules.insert("digits", vec![Rule::Digits(5)]);

    let result = validate(&rules, params);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().find(&["digits"]).unwrap(),
               &Value::U64(12345));
}

#[test]
fn test_digits_valid_string_negative() {
    let mut params = Map::new();
    params.assign("digits", Value::String("-12345".to_owned())).ok();

    let mut rules = BTreeMap::new();
    rules.insert("digits", vec![Rule::Digits(5)]);

    let result = validate(&rules, params);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().find(&["digits"]).unwrap(),
               &Value::I64(-12345));
}

#[test]
fn test_digits_valid_string_float() {
    let mut params = Map::new();
    params.assign("digits", Value::String("12345.67".to_owned())).ok();

    let mut rules = BTreeMap::new();
    rules.insert("digits", vec![Rule::Digits(5)]);

    let result = validate(&rules, params);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().find(&["digits"]).unwrap(),
               &Value::F64(12345.67));
}

#[test]
fn test_digits_invalid_string_digits() {
    let mut params = Map::new();
    params.assign("digits", Value::String("1234".to_owned())).ok();

    let mut rules = BTreeMap::new();
    rules.insert("digits", vec![Rule::Digits(5)]);

    let result = validate(&rules, params);

    assert!(result.is_err());
    assert_eq!(*result.unwrap_err().get("digits").unwrap(),
               vec!["The digits field must be a number with 5 digits.".to_owned()]);
}

#[test]
fn test_digits_invalid_string_non_numeric() {
    let mut params = Map::new();
    params.assign("digits", Value::String("fooba".to_owned())).ok();

    let mut rules = BTreeMap::new();
    rules.insert("digits", vec![Rule::Digits(5)]);

    let result = validate(&rules, params);

    assert!(result.is_err());
    assert_eq!(*result.unwrap_err().get("digits").unwrap(),
               vec!["The digits field must be a number with 5 digits.".to_owned()]);
}

#[test]
fn test_digits_valid_empty_string() {
    let mut params = Map::new();
    params.assign("digits", Value::String("".to_owned())).ok();

    let mut rules = BTreeMap::new();
    rules.insert("digits", vec![Rule::Digits(5)]);

    let result = validate(&rules, params);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().find(&["digits"]).unwrap(),
               &Value::String("".to_owned()));
}

#[test]
fn test_digits_valid_u64() {
    let mut params = Map::new();
    params.assign("digits", Value::U64(12345)).ok();

    let mut rules = BTreeMap::new();
    rules.insert("digits", vec![Rule::Digits(5)]);

    let result = validate(&rules, params);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().find(&["digits"]).unwrap(),
               &Value::U64(12345));
}

#[test]
fn test_digits_invalid_high_u64() {
    let mut params = Map::new();
    params.assign("digits", Value::U64(123456)).ok();

    let mut rules = BTreeMap::new();
    rules.insert("digits", vec![Rule::Digits(5)]);

    let result = validate(&rules, params);

    assert!(result.is_err());
    assert_eq!(*result.unwrap_err().get("digits").unwrap(),
               vec!["The digits field must be a number with 5 digits.".to_owned()]);
}

#[test]
fn test_digits_invalid_low_u64() {
    let mut params = Map::new();
    params.assign("digits", Value::U64(1234)).ok();

    let mut rules = BTreeMap::new();
    rules.insert("digits", vec![Rule::Digits(5)]);

    let result = validate(&rules, params);

    assert!(result.is_err());
    assert_eq!(*result.unwrap_err().get("digits").unwrap(),
               vec!["The digits field must be a number with 5 digits.".to_owned()]);
}

#[test]
fn test_digits_valid_i64() {
    let mut params = Map::new();
    params.assign("digits", Value::I64(-12345)).ok();

    let mut rules = BTreeMap::new();
    rules.insert("digits", vec![Rule::Digits(5)]);

    let result = validate(&rules, params);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().find(&["digits"]).unwrap(),
               &Value::I64(-12345));
}

#[test]
fn test_digits_invalid_high_i64() {
    let mut params = Map::new();
    params.assign("digits", Value::I64(-123456)).ok();

    let mut rules = BTreeMap::new();
    rules.insert("digits", vec![Rule::Digits(5)]);

    let result = validate(&rules, params);

    assert!(result.is_err());
    assert_eq!(*result.unwrap_err().get("digits").unwrap(),
               vec!["The digits field must be a number with 5 digits.".to_owned()]);
}

#[test]
fn test_digits_invalid_low_i64() {
    let mut params = Map::new();
    params.assign("digits", Value::I64(-1234)).ok();

    let mut rules = BTreeMap::new();
    rules.insert("digits", vec![Rule::Digits(5)]);

    let result = validate(&rules, params);

    assert!(result.is_err());
    assert_eq!(*result.unwrap_err().get("digits").unwrap(),
               vec!["The digits field must be a number with 5 digits.".to_owned()]);
}

#[test]
fn test_digits_valid_f64() {
    let mut params = Map::new();
    params.assign("digits", Value::F64(12345.67)).ok();

    let mut rules = BTreeMap::new();
    rules.insert("digits", vec![Rule::Digits(5)]);

    let result = validate(&rules, params);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().find(&["digits"]).unwrap(),
               &Value::F64(12345.67));
}

#[test]
fn test_digits_invalid_high_f64() {
    let mut params = Map::new();
    params.assign("digits", Value::F64(123456.7)).ok();

    let mut rules = BTreeMap::new();
    rules.insert("digits", vec![Rule::Digits(5)]);

    let result = validate(&rules, params);

    assert!(result.is_err());
    assert_eq!(*result.unwrap_err().get("digits").unwrap(),
               vec!["The digits field must be a number with 5 digits.".to_owned()]);
}

#[test]
fn test_digits_invalid_low_f64() {
    let mut params = Map::new();
    params.assign("digits", Value::F64(1234.567)).ok();

    let mut rules = BTreeMap::new();
    rules.insert("digits", vec![Rule::Digits(5)]);

    let result = validate(&rules, params);

    assert!(result.is_err());
    assert_eq!(*result.unwrap_err().get("digits").unwrap(),
               vec!["The digits field must be a number with 5 digits.".to_owned()]);
}

#[test]
fn test_digits_valid_blank() {
    let params = Map::new();

    let mut rules = BTreeMap::new();
    rules.insert("digits", vec![Rule::Digits(5)]);

    let result = validate(&rules, params);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().find(&["digits"]), None);
}

#[test]
fn test_digits_invalid_null() {
    let mut params = Map::new();
    params.assign("digits", Value::Null).ok();

    let mut rules = BTreeMap::new();
    rules.insert("digits", vec![Rule::Digits(5)]);

    let result = validate(&rules, params);

    assert!(result.is_err());
    assert_eq!(*result.unwrap_err().get("digits").unwrap(),
               vec!["The digits field must be a number with 5 digits.".to_owned()]);
}

#[test]
fn test_digits_valid_nested() {
    let mut test = Map::new();
    test.assign("digits", Value::String("12345".to_owned())).ok();
    let mut params = Map::new();
    params.assign("test", Value::Map(test)).ok();

    let mut rules = BTreeMap::new();
    rules.insert("test.digits", vec![Rule::Digits(5)]);

    let result = validate(&rules, params);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().find(&["test", "digits"]).unwrap(),
               &Value::U64(12345));
}
