extern crate iron_valid;
extern crate multipart;
extern crate params;

use iron_valid::{Rule, validate};
use params::{File, Map, Value};
use std::collections::BTreeMap;
use std::path::PathBuf;

#[test]
fn test_max_valid_string() {
    let mut params = Map::new();
    params.assign("size", Value::String("foo".to_owned())).ok();

    let mut rules = BTreeMap::new();
    rules.insert("size", vec![Rule::Max(3)]);

    let result = validate(rules, params);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().find(&["size"]).unwrap(),
               &Value::String("foo".to_owned()));
}

#[test]
fn test_max_valid_empty_string() {
    let mut params = Map::new();
    params.assign("size", Value::String("".to_owned())).ok();

    let mut rules = BTreeMap::new();
    rules.insert("size", vec![Rule::Max(2)]);

    let result = validate(rules, params);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().find(&["size"]).unwrap(),
               &Value::String("".to_owned()));
}

#[test]
fn test_max_invalid_high_string() {
    let mut params = Map::new();
    params.assign("size", Value::String("foobar".to_owned())).ok();

    let mut rules = BTreeMap::new();
    rules.insert("size", vec![Rule::Max(2)]);

    let result = validate(rules, params);

    assert!(result.is_err());
    assert_eq!(*result.unwrap_err().get("size").unwrap(),
               vec!["The size field must be no greater than 2 characters.".to_owned()]);
}

#[test]
fn test_max_valid_low_string() {
    let mut params = Map::new();
    params.assign("size", Value::String("foo".to_owned())).ok();

    let mut rules = BTreeMap::new();
    rules.insert("size", vec![Rule::Max(5)]);

    let result = validate(rules, params);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().find(&["size"]).unwrap(),
               &Value::String("foo".to_owned()));
}

#[test]
fn test_max_valid_u64() {
    let mut params = Map::new();
    params.assign("size", Value::U64(3)).ok();

    let mut rules = BTreeMap::new();
    rules.insert("size", vec![Rule::Max(3)]);

    let result = validate(rules, params);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().find(&["size"]).unwrap(), &Value::U64(3));
}

#[test]
fn test_max_invalid_high_u64() {
    let mut params = Map::new();
    params.assign("size", Value::U64(10)).ok();

    let mut rules = BTreeMap::new();
    rules.insert("size", vec![Rule::Max(5)]);

    let result = validate(rules, params);

    assert!(result.is_err());
    assert_eq!(*result.unwrap_err().get("size").unwrap(),
               vec!["The size field must be no greater than 5.".to_owned()]);
}

#[test]
fn test_max_valid_low_u64() {
    let mut params = Map::new();
    params.assign("size", Value::U64(1)).ok();

    let mut rules = BTreeMap::new();
    rules.insert("size", vec![Rule::Max(5)]);

    let result = validate(rules, params);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().find(&["size"]).unwrap(), &Value::U64(1));
}

#[test]
fn test_max_invalid_u64_below_0() {
    let mut params = Map::new();
    params.assign("size", Value::U64(1)).ok();

    let mut rules = BTreeMap::new();
    rules.insert("size", vec![Rule::Max(-5)]);

    let result = validate(rules, params);

    assert!(result.is_err());
    assert_eq!(*result.unwrap_err().get("size").unwrap(),
               vec!["The size field must be no greater than -5.".to_owned()]);
}

#[test]
fn test_max_valid_i64() {
    let mut params = Map::new();
    params.assign("size", Value::I64(3)).ok();

    let mut rules = BTreeMap::new();
    rules.insert("size", vec![Rule::Max(3)]);

    let result = validate(rules, params);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().find(&["size"]).unwrap(), &Value::I64(3));
}

#[test]
fn test_max_invalid_high_i64() {
    let mut params = Map::new();
    params.assign("size", Value::I64(10)).ok();

    let mut rules = BTreeMap::new();
    rules.insert("size", vec![Rule::Max(1)]);

    let result = validate(rules, params);

    assert!(result.is_err());
    assert_eq!(*result.unwrap_err().get("size").unwrap(),
               vec!["The size field must be no greater than 1.".to_owned()]);
}

#[test]
fn test_max_valid_low_i64() {
    let mut params = Map::new();
    params.assign("size", Value::I64(-10)).ok();

    let mut rules = BTreeMap::new();
    rules.insert("size", vec![Rule::Max(4)]);

    let result = validate(rules, params);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().find(&["size"]).unwrap(), &Value::I64(-10));
}

#[test]
fn test_max_valid_array() {
    let mut params = Map::new();
    params.assign("size", Value::Array(vec![Value::U64(1), Value::U64(2)])).ok();

    let mut rules = BTreeMap::new();
    rules.insert("size", vec![Rule::Max(2)]);

    let result = validate(rules, params);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().find(&["size"]).unwrap(),
               &Value::Array(vec![Value::U64(1), Value::U64(2)]));
}

#[test]
fn test_max_valid_empty_array() {
    let mut params = Map::new();
    params.assign("size", Value::Array(vec![])).ok();

    let mut rules = BTreeMap::new();
    rules.insert("size", vec![Rule::Max(4)]);

    let result = validate(rules, params);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().find(&["size"]).unwrap(),
               &Value::Array(vec![]));
}

#[test]
fn test_max_invalid_high_array() {
    let mut params = Map::new();
    params.assign("size",
                Value::Array(vec![Value::U64(1), Value::U64(2), Value::U64(3)]))
        .ok();

    let mut rules = BTreeMap::new();
    rules.insert("size", vec![Rule::Max(2)]);

    let result = validate(rules, params);

    assert!(result.is_err());
    assert_eq!(*result.unwrap_err().get("size").unwrap(),
               vec!["The size field must have no greater than 2 items.".to_owned()]);
}

#[test]
fn test_max_valid_low_array() {
    let mut params = Map::new();
    params.assign("size",
                Value::Array(vec![Value::U64(1), Value::U64(2), Value::U64(3)]))
        .ok();

    let mut rules = BTreeMap::new();
    rules.insert("size", vec![Rule::Max(8)]);

    let result = validate(rules, params);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().find(&["size"]).unwrap(),
               &Value::Array(vec![Value::U64(1), Value::U64(2), Value::U64(3)]));
}

#[test]
fn test_max_valid_object() {
    let mut params = Map::new();
    let mut items = Map::new();
    items.assign("foo", Value::U64(1)).ok();
    items.assign("bar", Value::U64(2)).ok();
    items.assign("baz", Value::U64(1)).ok();
    items.assign("qux", Value::U64(2)).ok();
    params.assign("size", Value::Map(items.clone())).ok();

    let mut rules = BTreeMap::new();
    rules.insert("size", vec![Rule::Max(4)]);

    let result = validate(rules, params);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().find(&["size"]).unwrap(), &Value::Map(items));
}

#[test]
fn test_max_valid_empty_object() {
    let mut params = Map::new();
    params.assign("size", Value::Map(Map::new())).ok();

    let mut rules = BTreeMap::new();
    rules.insert("size", vec![Rule::Max(4)]);

    let result = validate(rules, params);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().find(&["size"]).unwrap(),
               &Value::Map(Map::new()));
}

#[test]
fn test_max_invalid_high_object() {
    let mut params = Map::new();
    let mut items = Map::new();
    items.assign("foo", Value::U64(1)).ok();
    items.assign("bar", Value::U64(2)).ok();
    items.assign("baz", Value::U64(3)).ok();
    params.assign("size", Value::Map(items.clone())).ok();

    let mut rules = BTreeMap::new();
    rules.insert("size", vec![Rule::Max(2)]);

    let result = validate(rules, params);

    assert!(result.is_err());
    assert_eq!(*result.unwrap_err().get("size").unwrap(),
               vec!["The size field must have no greater than 2 items.".to_owned()]);
}

#[test]
fn test_max_valid_low_object() {
    let mut params = Map::new();
    let mut items = Map::new();
    items.assign("foo", Value::U64(1)).ok();
    items.assign("bar", Value::U64(2)).ok();
    items.assign("baz", Value::U64(3)).ok();
    params.assign("size", Value::Map(items.clone())).ok();

    let mut rules = BTreeMap::new();
    rules.insert("size", vec![Rule::Max(8)]);

    let result = validate(rules, params);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().find(&["size"]).unwrap(), &Value::Map(items));
}

#[test]
fn test_max_valid_file() {
    let mut params = Map::new();
    let file = File::from(multipart::server::SavedFile {
        path: PathBuf::from("files/temp_size_3914b.txt"),
        filename: None,
        size: 3914,
    });
    params.assign("size", Value::File(file.clone())).ok();

    let mut rules = BTreeMap::new();
    rules.insert("size", vec![Rule::Max(3)]);

    let result = validate(rules, params);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().find(&["size"]).unwrap(), &Value::File(file));
}

#[test]
fn test_max_invalid_high_file() {
    let mut params = Map::new();
    let file = File::from(multipart::server::SavedFile {
        path: PathBuf::from("files/temp_size_3914b.txt"),
        filename: None,
        size: 3914,
    });
    params.assign("size", Value::File(file.clone())).ok();

    let mut rules = BTreeMap::new();
    rules.insert("size", vec![Rule::Max(2)]);

    let result = validate(rules, params);

    assert!(result.is_err());
    assert_eq!(*result.unwrap_err().get("size").unwrap(),
               vec!["The size must be no greater than 2 kilobytes.".to_owned()]);
}

#[test]
fn test_max_valid_low_file() {
    let mut params = Map::new();
    let file = File::from(multipart::server::SavedFile {
        path: PathBuf::from("files/temp_size_3914b.txt"),
        filename: None,
        size: 3914,
    });
    params.assign("size", Value::File(file.clone())).ok();

    let mut rules = BTreeMap::new();
    rules.insert("size", vec![Rule::Max(8)]);

    let result = validate(rules, params);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().find(&["size"]).unwrap(), &Value::File(file));
}

#[test]
fn test_max_valid_blank() {
    let params = Map::new();

    let mut rules = BTreeMap::new();
    rules.insert("size", vec![Rule::Max(4)]);

    let result = validate(rules, params);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().find(&["size"]), None);
}

#[test]
fn test_max_invalid_null() {
    let mut params = Map::new();
    params.assign("size", Value::Null).ok();

    let mut rules = BTreeMap::new();
    rules.insert("size", vec![Rule::Max(4)]);

    let result = validate(rules, params);

    assert!(result.is_err());
    assert_eq!(*result.unwrap_err().get("size").unwrap(),
               vec!["The size field must have no greater than a size of 4.".to_owned()]);
}
