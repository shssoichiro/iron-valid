extern crate iron_valid;
extern crate params;

use iron_valid::{Rule, validate};
use params::{Map, Value};
use std::collections::BTreeMap;

#[test]
fn test_ip_address_valid() {
    let mut params = Map::new();
    params.assign("ip", Value::String("192.168.1.1".to_owned())).ok();

    let mut rules = BTreeMap::new();
    rules.insert("ip", vec![Rule::IpAddress]);

    let result = validate(rules, params);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().find(&["ip"]).unwrap(),
               &Value::String("192.168.1.1".to_owned()));
}

#[test]
fn test_ip_address_valid_v6() {
    let mut params = Map::new();
    params.assign("ip",
                Value::String("2001:0db8:0000:0042:0000:8a2e:0370:7334".to_owned()))
        .ok();

    let mut rules = BTreeMap::new();
    rules.insert("ip", vec![Rule::IpAddress]);

    let result = validate(rules, params);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().find(&["ip"]).unwrap(),
               &Value::String("2001:0db8:0000:0042:0000:8a2e:0370:7334".to_owned()));
}

#[test]
fn test_ip_address_invalid_out_of_range() {
    let mut params = Map::new();
    params.assign("ip", Value::String("256.0.0.0".to_owned())).ok();

    let mut rules = BTreeMap::new();
    rules.insert("ip", vec![Rule::IpAddress]);

    let result = validate(rules, params);

    assert!(result.is_err());
    assert_eq!(*result.unwrap_err().get("ip").unwrap(),
               vec!["The ip field must contain a valid IP address.".to_owned()]);
}

#[test]
fn test_ip_address_invalid_string() {
    let mut params = Map::new();
    params.assign("ip", Value::String("foobar.com".to_owned())).ok();

    let mut rules = BTreeMap::new();
    rules.insert("ip", vec![Rule::IpAddress]);

    let result = validate(rules, params);

    assert!(result.is_err());
    assert_eq!(*result.unwrap_err().get("ip").unwrap(),
               vec!["The ip field must contain a valid IP address.".to_owned()]);
}

#[test]
fn test_ip_address_invalid_numeric() {
    let mut params = Map::new();
    params.assign("ip", Value::U64(42)).ok();

    let mut rules = BTreeMap::new();
    rules.insert("ip", vec![Rule::IpAddress]);

    let result = validate(rules, params);

    assert!(result.is_err());
    assert_eq!(*result.unwrap_err().get("ip").unwrap(),
               vec!["The ip field must contain a valid IP address.".to_owned()]);
}

#[test]
fn test_ip_address_valid_empty() {
    let mut params = Map::new();
    params.assign("ip", Value::String("".to_owned())).ok();

    let mut rules = BTreeMap::new();
    rules.insert("ip", vec![Rule::IpAddress]);

    let result = validate(rules, params);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().find(&["ip"]).unwrap(),
               &Value::String("".to_owned()));
}

#[test]
fn test_ip_address_valid_blank() {
    let params = Map::new();

    let mut rules = BTreeMap::new();
    rules.insert("ip", vec![Rule::IpAddress]);

    let result = validate(rules, params);

    assert!(result.is_ok());
    assert_eq!(result.unwrap().find(&["ip"]), None);
}

#[test]
fn test_ip_address_invalid_null() {
    let mut params = Map::new();
    params.assign("ip", Value::Null).ok();

    let mut rules = BTreeMap::new();
    rules.insert("ip", vec![Rule::IpAddress]);

    let result = validate(rules, params);

    assert!(result.is_err());
    assert_eq!(*result.unwrap_err().get("ip").unwrap(),
               vec!["The ip field must contain a valid IP address.".to_owned()]);
}
