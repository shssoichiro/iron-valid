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
fn test_size_valid_string() {
    let mut params = Map::new();
    params.assign("size", Value::String("foo".to_owned())).ok();

    let mut rules = BTreeMap::new();
    rules.insert("size", vec![Rule::Size(3)]);

    let result = validate(&rules, params);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().find(&["size"]).unwrap(),
               &Value::String("foo".to_owned()));
}

#[test]
fn test_size_valid_empty_string() {
    let mut params = Map::new();
    params.assign("size", Value::String("".to_owned())).ok();

    let mut rules = BTreeMap::new();
    rules.insert("size", vec![Rule::Size(2)]);

    let result = validate(&rules, params);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().find(&["size"]).unwrap(),
               &Value::String("".to_owned()));
}

#[test]
fn test_size_invalid_high_string() {
    let mut params = Map::new();
    params.assign("size", Value::String("foobar".to_owned())).ok();

    let mut rules = BTreeMap::new();
    rules.insert("size", vec![Rule::Size(2)]);

    let result = validate(&rules, params);

    assert!(result.is_err());
    assert_eq!(*result.unwrap_err().get("size").unwrap(),
               vec!["The size field must be 2 characters.".to_owned()]);
}

#[test]
fn test_size_invalid_low_string() {
    let mut params = Map::new();
    params.assign("size", Value::String("foo".to_owned())).ok();

    let mut rules = BTreeMap::new();
    rules.insert("size", vec![Rule::Size(5)]);

    let result = validate(&rules, params);

    assert!(result.is_err());
    assert_eq!(*result.unwrap_err().get("size").unwrap(),
               vec!["The size field must be 5 characters.".to_owned()]);
}

#[test]
fn test_size_valid_u64() {
    let mut params = Map::new();
    params.assign("size", Value::U64(3)).ok();

    let mut rules = BTreeMap::new();
    rules.insert("size", vec![Rule::Size(3)]);

    let result = validate(&rules, params);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().find(&["size"]).unwrap(), &Value::U64(3));
}

#[test]
fn test_size_invalid_high_u64() {
    let mut params = Map::new();
    params.assign("size", Value::U64(10)).ok();

    let mut rules = BTreeMap::new();
    rules.insert("size", vec![Rule::Size(5)]);

    let result = validate(&rules, params);

    assert!(result.is_err());
    assert_eq!(*result.unwrap_err().get("size").unwrap(),
               vec!["The size field must be 5.".to_owned()]);
}

#[test]
fn test_size_invalid_low_u64() {
    let mut params = Map::new();
    params.assign("size", Value::U64(1)).ok();

    let mut rules = BTreeMap::new();
    rules.insert("size", vec![Rule::Size(5)]);

    let result = validate(&rules, params);

    assert!(result.is_err());
    assert_eq!(*result.unwrap_err().get("size").unwrap(),
               vec!["The size field must be 5.".to_owned()]);
}

#[test]
fn test_size_invalid_u64_below_0() {
    let mut params = Map::new();
    params.assign("size", Value::U64(1)).ok();

    let mut rules = BTreeMap::new();
    rules.insert("size", vec![Rule::Size(-5)]);

    let result = validate(&rules, params);

    assert!(result.is_err());
    assert_eq!(*result.unwrap_err().get("size").unwrap(),
               vec!["The size field must be -5.".to_owned()]);
}

#[test]
fn test_size_valid_i64() {
    let mut params = Map::new();
    params.assign("size", Value::I64(3)).ok();

    let mut rules = BTreeMap::new();
    rules.insert("size", vec![Rule::Size(3)]);

    let result = validate(&rules, params);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().find(&["size"]).unwrap(), &Value::I64(3));
}

#[test]
fn test_size_invalid_high_i64() {
    let mut params = Map::new();
    params.assign("size", Value::I64(10)).ok();

    let mut rules = BTreeMap::new();
    rules.insert("size", vec![Rule::Size(1)]);

    let result = validate(&rules, params);

    assert!(result.is_err());
    assert_eq!(*result.unwrap_err().get("size").unwrap(),
               vec!["The size field must be 1.".to_owned()]);
}

#[test]
fn test_size_invalid_low_i64() {
    let mut params = Map::new();
    params.assign("size", Value::I64(-10)).ok();

    let mut rules = BTreeMap::new();
    rules.insert("size", vec![Rule::Size(4)]);

    let result = validate(&rules, params);

    assert!(result.is_err());
    assert_eq!(*result.unwrap_err().get("size").unwrap(),
               vec!["The size field must be 4.".to_owned()]);
}

#[test]
fn test_size_valid_array() {
    let mut params = Map::new();
    params.assign("size", Value::Array(vec![Value::U64(1), Value::U64(2)])).ok();

    let mut rules = BTreeMap::new();
    rules.insert("size", vec![Rule::Size(2)]);

    let result = validate(&rules, params);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().find(&["size"]).unwrap(),
               &Value::Array(vec![Value::U64(1), Value::U64(2)]));
}

#[test]
fn test_size_valid_empty_array() {
    let mut params = Map::new();
    params.assign("size", Value::Array(vec![])).ok();

    let mut rules = BTreeMap::new();
    rules.insert("size", vec![Rule::Size(4)]);

    let result = validate(&rules, params);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().find(&["size"]).unwrap(),
               &Value::Array(vec![]));
}

#[test]
fn test_size_invalid_high_array() {
    let mut params = Map::new();
    params.assign("size",
                  Value::Array(vec![Value::U64(1), Value::U64(2), Value::U64(3)]))
        .ok();

    let mut rules = BTreeMap::new();
    rules.insert("size", vec![Rule::Size(2)]);

    let result = validate(&rules, params);

    assert!(result.is_err());
    assert_eq!(*result.unwrap_err().get("size").unwrap(),
               vec!["The size field must have 2 items.".to_owned()]);
}

#[test]
fn test_size_invalid_low_array() {
    let mut params = Map::new();
    params.assign("size",
                  Value::Array(vec![Value::U64(1), Value::U64(2), Value::U64(3)]))
        .ok();

    let mut rules = BTreeMap::new();
    rules.insert("size", vec![Rule::Size(8)]);

    let result = validate(&rules, params);

    assert!(result.is_err());
    assert_eq!(*result.unwrap_err().get("size").unwrap(),
               vec!["The size field must have 8 items.".to_owned()]);
}

#[test]
fn test_size_valid_object() {
    let mut params = Map::new();
    let mut items = Map::new();
    items.assign("foo", Value::U64(1)).ok();
    items.assign("bar", Value::U64(2)).ok();
    items.assign("baz", Value::U64(1)).ok();
    items.assign("qux", Value::U64(2)).ok();
    params.assign("size", Value::Map(items.clone())).ok();

    let mut rules = BTreeMap::new();
    rules.insert("size", vec![Rule::Size(4)]);

    let result = validate(&rules, params);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().find(&["size"]).unwrap(), &Value::Map(items));
}

#[test]
fn test_size_valid_empty_object() {
    let mut params = Map::new();
    params.assign("size", Value::Map(Map::new())).ok();

    let mut rules = BTreeMap::new();
    rules.insert("size", vec![Rule::Size(4)]);

    let result = validate(&rules, params);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().find(&["size"]).unwrap(),
               &Value::Map(Map::new()));
}

#[test]
fn test_size_invalid_high_object() {
    let mut params = Map::new();
    let mut items = Map::new();
    items.assign("foo", Value::U64(1)).ok();
    items.assign("bar", Value::U64(2)).ok();
    items.assign("baz", Value::U64(3)).ok();
    params.assign("size", Value::Map(items.clone())).ok();

    let mut rules = BTreeMap::new();
    rules.insert("size", vec![Rule::Size(2)]);

    let result = validate(&rules, params);

    assert!(result.is_err());
    assert_eq!(*result.unwrap_err().get("size").unwrap(),
               vec!["The size field must have 2 items.".to_owned()]);
}

#[test]
fn test_size_invalid_low_object() {
    let mut params = Map::new();
    let mut items = Map::new();
    items.assign("foo", Value::U64(1)).ok();
    items.assign("bar", Value::U64(2)).ok();
    items.assign("baz", Value::U64(3)).ok();
    params.assign("size", Value::Map(items.clone())).ok();

    let mut rules = BTreeMap::new();
    rules.insert("size", vec![Rule::Size(8)]);

    let result = validate(&rules, params);

    assert!(result.is_err());
    assert_eq!(*result.unwrap_err().get("size").unwrap(),
               vec!["The size field must have 8 items.".to_owned()]);
}

#[test]
fn test_size_valid_file() {
    let mut params = Map::new();
    let file = File {
        path: PathBuf::from("files/temp_size_3914b.txt"),
        filename: None,
        size: 3914,
        content_type: Mime::from_str("text/plain").unwrap(),
    };
    params.assign("size", Value::File(file.clone())).ok();

    let mut rules = BTreeMap::new();
    rules.insert("size", vec![Rule::Size(3)]);

    let result = validate(&rules, params);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().find(&["size"]).unwrap(), &Value::File(file));
}

#[test]
fn test_size_invalid_high_file() {
    let mut params = Map::new();
    let file = File {
        path: PathBuf::from("files/temp_size_3914b.txt"),
        filename: None,
        size: 3914,
        content_type: Mime::from_str("text/plain").unwrap(),
    };
    params.assign("size", Value::File(file.clone())).ok();

    let mut rules = BTreeMap::new();
    rules.insert("size", vec![Rule::Size(2)]);

    let result = validate(&rules, params);

    assert!(result.is_err());
    assert_eq!(*result.unwrap_err().get("size").unwrap(),
               vec!["The size must be 2 kilobytes.".to_owned()]);
}

#[test]
fn test_size_invalid_low_file() {
    let mut params = Map::new();
    let file = File {
        path: PathBuf::from("files/temp_size_3914b.txt"),
        filename: None,
        size: 3914,
        content_type: Mime::from_str("text/plain").unwrap(),
    };
    params.assign("size", Value::File(file.clone())).ok();

    let mut rules = BTreeMap::new();
    rules.insert("size", vec![Rule::Size(8)]);

    let result = validate(&rules, params);

    assert!(result.is_err());
    assert_eq!(*result.unwrap_err().get("size").unwrap(),
               vec!["The size must be 8 kilobytes.".to_owned()]);
}

#[test]
fn test_size_valid_blank() {
    let params = Map::new();

    let mut rules = BTreeMap::new();
    rules.insert("size", vec![Rule::Size(4)]);

    let result = validate(&rules, params);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().find(&["size"]), None);
}

#[test]
fn test_size_invalid_null() {
    let mut params = Map::new();
    params.assign("size", Value::Null).ok();

    let mut rules = BTreeMap::new();
    rules.insert("size", vec![Rule::Size(4)]);

    let result = validate(&rules, params);

    assert!(result.is_err());
    assert_eq!(*result.unwrap_err().get("size").unwrap(),
               vec!["The size field must have a size of 4.".to_owned()]);
}

#[test]
fn test_size_valid_nested() {
    let mut test = Map::new();
    test.assign("size", Value::String("foo".to_owned())).ok();
    let mut params = Map::new();
    params.assign("test", Value::Map(test)).ok();

    let mut rules = BTreeMap::new();
    rules.insert("test.size", vec![Rule::Size(3)]);

    let result = validate(&rules, params);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().find(&["test", "size"]).unwrap(),
               &Value::String("foo".to_owned()));
}
