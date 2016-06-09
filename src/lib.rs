#[macro_use]
extern crate lazy_static;
extern crate params;
extern crate regex;

use params::{Map, Value};
use std::collections::BTreeMap;

mod validators {
    pub mod accepted;
    pub mod email;
}

#[derive(Debug,Clone)]
pub enum Rule {
    /// The field under validation must be `yes`, `on`, `1`, or `true`.
    /// This is useful for validating "Terms of Service" acceptance.
    ///
    /// On success, will transform the input to a boolean `true`.
    Accepted,
    /// The field under validation must be a valid URL with an active DNS entry.
    ActiveUrl,
    /// The field under validation must be entirely alphabetic characters.
    Alpha,
    /// The field under validation may have alpha-numeric characters,
    /// as well as dashes and underscores.
    AlphaDash,
    /// The field under validation must be entirely alpha-numeric characters.
    AlphaNumeric,
    /// The field under validation must be an array.
    Array,
    /// The field under validation must have a size between the given min and max.
    /// Strings, numerics, and files are evaluated in the same fashion as the `Size` rule.
    Between(isize, isize),
    /// The field under validation must be able to be cast as a boolean.
    /// Accepted input are `true`, `false`, `1`, `0`, `"1"`, and `"0"`.
    Boolean,
    /// The field under validation must have a matching field of `foo_confirmation`.
    /// For example, if the field under validation is `password`,
    /// a matching `password_confirmation` field must be present in the input.
    Confirmed,
    /// The field under validation must have a different value than `field`.
    Different(String),
    /// The field under validation must be numeric and must have an exact length of value.
    Digits(usize),
    /// The field under validation must have a length between the given min and max.
    DigitsBetween(usize, usize),
    /// When working with arrays, the field under validation must not have any duplicate values.
    Distinct,
    /// The field under validation must be formatted as an e-mail address.
    Email,
    /// The field under validation must not be empty when it is present.
    Filled,
    /// The field under validation must be included in the given list of values.
    In(Vec<Value>),
    /// The field under validation must exist in `anotherfield`'s values.
    InArray(String),
    /// The field under validation must be an integer.
    Integer,
    /// The field under validation must be an IP address.
    IpAddress,
    /// The field under validation must be a valid JSON string.
    Json,
    /// The field under validation must be less than or equal to a maximum value.
    /// Strings, numerics, and files are evaluated in the same fashion as the `Size` rule.
    Max(isize),
    /// The field under validation must have a minimum value.
    /// Strings, numerics, and files are evaluated in the same fashion as the `Size` rule.
    Min(isize),
    /// The field under validation must not be included in the given list of values.
    NotIn(Vec<Value>),
    /// The field under validation must not exist in `anotherfield`'s values.
    NotInArray(String),
    /// The field under validation must be numeric.
    Numeric,
    /// The field under validation must be present in the input data but can be empty.
    Present,
    /// The field under validation must match the given regular expression.
    Regex(String),
    /// The field under validation must be present in the input data and not empty.
    /// A field is considered "empty" if one of the following conditions are true:
    ///
    /// The value is null.
    ///
    /// The value is an empty string.
    ///
    /// The value is an empty array or empty object.
    Required,
    /// The field under validation must be present if the `anotherfield` field
    /// is equal to any `value`.
    RequiredIf(String, Value),
    /// The field under validation must be present unless the `anotherfield` field
    /// is equal to any `value`.
    RequiredUnless(String, Value),
    /// The field under validation must be present only if
    /// any of the other specified fields are present.
    RequiredWith(Vec<String>),
    /// The field under validation must be present only if
    /// all of the other specified fields are present.
    RequiredWithAll(Vec<String>),
    /// The field under validation must be present only when
    /// any of the other specified fields are not present.
    RequiredWithout(Vec<String>),
    /// The field under validation must be present only when
    /// all of the other specified fields are not present.
    RequiredWithoutAll(Vec<String>),
    /// The given field must match the field under validation.
    Same(String),
    /// The field under validation must have a size matching the given value.
    ///
    /// For string data, value corresponds to the number of characters.
    ///
    /// For numeric data, value corresponds to a given integer value.
    ///
    /// For files, size corresponds to the file size in kilobytes.
    Size(isize),
    /// The field under validation must be a string.
    String,
    /// The field under validation must be formatted as a valid URL,
    /// but does not need to resolve to a real website.
    Url,
}

pub fn validate(rules: BTreeMap<&str, Vec<Rule>>,
                values: Map)
                -> Result<Map, BTreeMap<&str, Vec<String>>> {
    let mut new_values = values;
    let mut errors = BTreeMap::new();

    for (field, ruleset) in &rules {
        let mut current_errors = Vec::new();
        for rule in ruleset {
            let result = match *rule {
                Rule::Accepted => validators::accepted::validate_accepted(&new_values, field),
                Rule::Email => validators::email::validate_email(&new_values, field),
                _ => {
                    panic!(format!("Unrecognized validation rule {:?} for field {:?}",
                                   rule,
                                   field));
                }
            };
            match result {
                Ok(Some(res)) => {
                    new_values.assign(field, res).ok();
                }
                Ok(None) => (),
                Err(err) => {
                    current_errors.push(err);
                }
            };
        }
        if !current_errors.is_empty() {
            errors.insert(*field, current_errors);
        }
    }

    if errors.is_empty() {
        Ok(new_values)
    } else {
        Err(errors)
    }
}
