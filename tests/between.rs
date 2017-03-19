extern crate iron_valid;
extern crate mime;
extern crate params;

use iron_valid::{Rule, validate};
use mime::Mime;
use params::{File, Map, Value};
use std::collections::BTreeMap;
use std::path::PathBuf;
use std::str::FromStr;

#[test]
fn test_between_valid_string() {
    let mut params = Map::new();
    params.assign("between", Value::String("foo".to_owned())).ok();

    let mut rules = BTreeMap::new();
    rules.insert("between", vec![Rule::Between(1, 5)]);

    let result = validate(&rules, params);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().find(&["between"]).unwrap(),
               &Value::String("foo".to_owned()));
}

#[test]
fn test_between_valid_empty_string() {
    let mut params = Map::new();
    params.assign("between", Value::String("".to_owned())).ok();

    let mut rules = BTreeMap::new();
    rules.insert("between", vec![Rule::Between(1, 5)]);

    let result = validate(&rules, params);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().find(&["between"]).unwrap(),
               &Value::String("".to_owned()));
}

#[test]
fn test_between_invalid_high_string() {
    let mut params = Map::new();
    params.assign("between", Value::String("foobar".to_owned())).ok();

    let mut rules = BTreeMap::new();
    rules.insert("between", vec![Rule::Between(1, 5)]);

    let result = validate(&rules, params);

    assert!(result.is_err());
    assert_eq!(*result.unwrap_err().get("between").unwrap(),
               vec!["The between field must be between 1 and 5 characters.".to_owned()]);
}

#[test]
fn test_between_invalid_low_string() {
    let mut params = Map::new();
    params.assign("between", Value::String("foo".to_owned())).ok();

    let mut rules = BTreeMap::new();
    rules.insert("between", vec![Rule::Between(5, 10)]);

    let result = validate(&rules, params);

    assert!(result.is_err());
    assert_eq!(*result.unwrap_err().get("between").unwrap(),
               vec!["The between field must be between 5 and 10 characters.".to_owned()]);
}

#[test]
fn test_between_valid_u64() {
    let mut params = Map::new();
    params.assign("between", Value::U64(3)).ok();

    let mut rules = BTreeMap::new();
    rules.insert("between", vec![Rule::Between(1, 5)]);

    let result = validate(&rules, params);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().find(&["between"]).unwrap(), &Value::U64(3));
}

#[test]
fn test_between_invalid_high_u64() {
    let mut params = Map::new();
    params.assign("between", Value::U64(10)).ok();

    let mut rules = BTreeMap::new();
    rules.insert("between", vec![Rule::Between(1, 5)]);

    let result = validate(&rules, params);

    assert!(result.is_err());
    assert_eq!(*result.unwrap_err().get("between").unwrap(),
               vec!["The between field must be between 1 and 5.".to_owned()]);
}

#[test]
fn test_between_invalid_low_u64() {
    let mut params = Map::new();
    params.assign("between", Value::U64(1)).ok();

    let mut rules = BTreeMap::new();
    rules.insert("between", vec![Rule::Between(2, 5)]);

    let result = validate(&rules, params);

    assert!(result.is_err());
    assert_eq!(*result.unwrap_err().get("between").unwrap(),
               vec!["The between field must be between 2 and 5.".to_owned()]);
}

#[test]
fn test_between_valid_u64_min_below_0() {
    let mut params = Map::new();
    params.assign("between", Value::U64(3)).ok();

    let mut rules = BTreeMap::new();
    rules.insert("between", vec![Rule::Between(-1, 5)]);

    let result = validate(&rules, params);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().find(&["between"]).unwrap(), &Value::U64(3));
}

#[test]
fn test_between_invalid_u64_max_below_0() {
    let mut params = Map::new();
    params.assign("between", Value::U64(1)).ok();

    let mut rules = BTreeMap::new();
    rules.insert("between", vec![Rule::Between(-5, -1)]);

    let result = validate(&rules, params);

    assert!(result.is_err());
    assert_eq!(*result.unwrap_err().get("between").unwrap(),
               vec!["The between field must be between -5 and -1.".to_owned()]);
}

#[test]
fn test_between_valid_i64() {
    let mut params = Map::new();
    params.assign("between", Value::I64(3)).ok();

    let mut rules = BTreeMap::new();
    rules.insert("between", vec![Rule::Between(1, 5)]);

    let result = validate(&rules, params);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().find(&["between"]).unwrap(), &Value::I64(3));
}

#[test]
fn test_between_invalid_high_i64() {
    let mut params = Map::new();
    params.assign("between", Value::I64(10)).ok();

    let mut rules = BTreeMap::new();
    rules.insert("between", vec![Rule::Between(1, 5)]);

    let result = validate(&rules, params);

    assert!(result.is_err());
    assert_eq!(*result.unwrap_err().get("between").unwrap(),
               vec!["The between field must be between 1 and 5.".to_owned()]);
}

#[test]
fn test_between_invalid_low_i64() {
    let mut params = Map::new();
    params.assign("between", Value::I64(-10)).ok();

    let mut rules = BTreeMap::new();
    rules.insert("between", vec![Rule::Between(1, 5)]);

    let result = validate(&rules, params);

    assert!(result.is_err());
    assert_eq!(*result.unwrap_err().get("between").unwrap(),
               vec!["The between field must be between 1 and 5.".to_owned()]);
}

#[test]
fn test_between_valid_array() {
    let mut params = Map::new();
    params.assign("between", Value::Array(vec![Value::U64(1), Value::U64(2)])).ok();

    let mut rules = BTreeMap::new();
    rules.insert("between", vec![Rule::Between(1, 5)]);

    let result = validate(&rules, params);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().find(&["between"]).unwrap(),
               &Value::Array(vec![Value::U64(1), Value::U64(2)]));
}

#[test]
fn test_between_valid_empty_array() {
    let mut params = Map::new();
    params.assign("between", Value::Array(vec![])).ok();

    let mut rules = BTreeMap::new();
    rules.insert("between", vec![Rule::Between(1, 5)]);

    let result = validate(&rules, params);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().find(&["between"]).unwrap(),
               &Value::Array(vec![]));
}

#[test]
fn test_between_invalid_high_array() {
    let mut params = Map::new();
    params.assign("between",
                  Value::Array(vec![Value::U64(1), Value::U64(2), Value::U64(3)]))
        .ok();

    let mut rules = BTreeMap::new();
    rules.insert("between", vec![Rule::Between(1, 2)]);

    let result = validate(&rules, params);

    assert!(result.is_err());
    assert_eq!(*result.unwrap_err().get("between").unwrap(),
               vec!["The between field must have between 1 and 2 items.".to_owned()]);
}

#[test]
fn test_between_invalid_low_array() {
    let mut params = Map::new();
    params.assign("between",
                  Value::Array(vec![Value::U64(1), Value::U64(2), Value::U64(3)]))
        .ok();

    let mut rules = BTreeMap::new();
    rules.insert("between", vec![Rule::Between(5, 8)]);

    let result = validate(&rules, params);

    assert!(result.is_err());
    assert_eq!(*result.unwrap_err().get("between").unwrap(),
               vec!["The between field must have between 5 and 8 items.".to_owned()]);
}

#[test]
fn test_between_valid_object() {
    let mut params = Map::new();
    let mut items = Map::new();
    items.assign("foo", Value::U64(1)).ok();
    items.assign("bar", Value::U64(2)).ok();
    params.assign("between", Value::Map(items.clone())).ok();

    let mut rules = BTreeMap::new();
    rules.insert("between", vec![Rule::Between(1, 5)]);

    let result = validate(&rules, params);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().find(&["between"]).unwrap(),
               &Value::Map(items));
}

#[test]
fn test_between_valid_empty_object() {
    let mut params = Map::new();
    params.assign("between", Value::Map(Map::new())).ok();

    let mut rules = BTreeMap::new();
    rules.insert("between", vec![Rule::Between(1, 5)]);

    let result = validate(&rules, params);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().find(&["between"]).unwrap(),
               &Value::Map(Map::new()));
}

#[test]
fn test_between_invalid_high_object() {
    let mut params = Map::new();
    let mut items = Map::new();
    items.assign("foo", Value::U64(1)).ok();
    items.assign("bar", Value::U64(2)).ok();
    items.assign("baz", Value::U64(3)).ok();
    params.assign("between", Value::Map(items.clone())).ok();

    let mut rules = BTreeMap::new();
    rules.insert("between", vec![Rule::Between(1, 2)]);

    let result = validate(&rules, params);

    assert!(result.is_err());
    assert_eq!(*result.unwrap_err().get("between").unwrap(),
               vec!["The between field must have between 1 and 2 items.".to_owned()]);
}

#[test]
fn test_between_invalid_low_object() {
    let mut params = Map::new();
    let mut items = Map::new();
    items.assign("foo", Value::U64(1)).ok();
    items.assign("bar", Value::U64(2)).ok();
    items.assign("baz", Value::U64(3)).ok();
    params.assign("between", Value::Map(items.clone())).ok();

    let mut rules = BTreeMap::new();
    rules.insert("between", vec![Rule::Between(5, 8)]);

    let result = validate(&rules, params);

    assert!(result.is_err());
    assert_eq!(*result.unwrap_err().get("between").unwrap(),
               vec!["The between field must have between 5 and 8 items.".to_owned()]);
}

#[test]
fn test_between_valid_file() {
    let mut params = Map::new();
    let file = File {
        path: PathBuf::from("files/temp_size_3914b.txt"),
        filename: None,
        size: 3914,
        content_type: Mime::from_str("text/plain").unwrap(),
    };
    params.assign("between", Value::File(file.clone())).ok();

    let mut rules = BTreeMap::new();
    rules.insert("between", vec![Rule::Between(1, 5)]);

    let result = validate(&rules, params);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().find(&["between"]).unwrap(),
               &Value::File(file));
}

#[test]
fn test_between_invalid_high_file() {
    let mut params = Map::new();
    let file = File {
        path: PathBuf::from("files/temp_size_3914b.txt"),
        filename: None,
        size: 3914,
        content_type: Mime::from_str("text/plain").unwrap(),
    };
    params.assign("between", Value::File(file.clone())).ok();

    let mut rules = BTreeMap::new();
    rules.insert("between", vec![Rule::Between(1, 2)]);

    let result = validate(&rules, params);

    assert!(result.is_err());
    assert_eq!(*result.unwrap_err().get("between").unwrap(),
               vec!["The between must be between 1 and 2 kilobytes.".to_owned()]);
}

#[test]
fn test_between_invalid_low_file() {
    let mut params = Map::new();
    let file = File {
        path: PathBuf::from("files/temp_size_3914b.txt"),
        filename: None,
        size: 3914,
        content_type: Mime::from_str("text/plain").unwrap(),
    };
    params.assign("between", Value::File(file.clone())).ok();

    let mut rules = BTreeMap::new();
    rules.insert("between", vec![Rule::Between(5, 8)]);

    let result = validate(&rules, params);

    assert!(result.is_err());
    assert_eq!(*result.unwrap_err().get("between").unwrap(),
               vec!["The between must be between 5 and 8 kilobytes.".to_owned()]);
}

#[test]
fn test_between_valid_blank() {
    let params = Map::new();

    let mut rules = BTreeMap::new();
    rules.insert("between", vec![Rule::Between(0, 1)]);

    let result = validate(&rules, params);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().find(&["between"]), None);
}

#[test]
fn test_between_invalid_null() {
    let mut params = Map::new();
    params.assign("between", Value::Null).ok();

    let mut rules = BTreeMap::new();
    rules.insert("between", vec![Rule::Between(0, 1)]);

    let result = validate(&rules, params);

    assert!(result.is_err());
    assert_eq!(*result.unwrap_err().get("between").unwrap(),
               vec!["The between field must have a size between 0 and 1.".to_owned()]);
}

#[test]
fn test_between_valid_nested() {
    let mut test = Map::new();
    test.assign("between", Value::String("foo".to_owned())).ok();
    let mut params = Map::new();
    params.assign("test", Value::Map(test)).ok();

    let mut rules = BTreeMap::new();
    rules.insert("test.between", vec![Rule::Between(1, 5)]);

    let result = validate(&rules, params);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().find(&["test", "between"]).unwrap(),
               &Value::String("foo".to_owned()));
}
