#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]

extern crate dns_lookup;
#[macro_use]
extern crate lazy_static;
extern crate params;
#[cfg(feature = "pg")]
extern crate postgres;
extern crate regex;
extern crate serde_json;
extern crate url;

use params::{Map, Value};
use std::collections::BTreeMap;

mod validators {
    pub mod accepted;
    pub mod active_url;
    pub mod alpha;
    pub mod alpha_dash;
    pub mod alpha_numeric;
    pub mod array;
    pub mod between;
    pub mod boolean;
    pub mod confirmed;
    pub mod different;
    pub mod digits;
    pub mod digits_between;
    pub mod distinct;
    pub mod email;
    #[cfg(feature = "pg")]
    pub mod exists;
    pub mod filled;
    pub mod in_const;
    pub mod in_array;
    pub mod integer;
    pub mod ip_address;
    pub mod json;
    pub mod max;
    pub mod min;
    pub mod not_in;
    pub mod not_in_array;
    pub mod numeric;
    pub mod present;
    pub mod regex;
    pub mod required;
    pub mod required_if;
    pub mod required_unless;
    pub mod required_with;
    pub mod required_with_all;
    pub mod required_without;
    pub mod required_without_all;
    pub mod same;
    pub mod size;
    pub mod string;
    #[cfg(feature = "pg")]
    pub mod unique;
    pub mod url;
}

#[derive(Debug,Clone)]
pub enum Rule<'a> {
    /// The field under validation must be `yes`, `on`, `1`, or `true`.
    /// This is useful for validating "Terms of Service" acceptance.
    ///
    /// On success, will transform the input to a boolean `true`.
    Accepted,
    /// The field under validation, if present, must be an active domain name.
    ActiveUrl,
    /// The field under validation must be entirely alphabetic characters.
    ///
    /// This validator accepts Latin and international (Unicode) input.
    Alpha,
    /// The field under validation may have alpha-numeric characters,
    /// as well as dashes and underscores.
    ///
    /// This validator accepts Latin and international (Unicode) input.
    AlphaDash,
    /// The field under validation must be entirely alpha-numeric characters.
    ///
    /// This validator accepts Latin and international (Unicode) input.
    AlphaNumeric,
    /// The field under validation, if present, must be an array.
    Array,
    /// The field under validation, if present, must have a size between the given min and max.
    /// Strings, numerics, and files are evaluated in the same fashion as the `Size` rule.
    Between(isize, isize),
    /// The field under validation, if present, must be able to be cast as a boolean.
    /// Accepted input are `true`, `false`, `1`, `0`, `"1"`, and `"0"`.
    ///
    /// On success, will transform the input to a boolean `true` or `false`.
    Boolean,
    /// The field under validation must have a matching field of `foo_confirmation`.
    /// For example, if the field under validation is `password`,
    /// a matching `password_confirmation` field must be present in the input.
    Confirmed,
    /// The field under validation must have a different value than `field`.
    Different(&'static str),
    /// The field under validation, if present,
    /// must be numeric and must have an exact length of value.
    ///
    /// For negative numbers, the negative sign is not counted as a digit.
    /// For floating point numbers, the number of digits preceeding the decimal place
    /// is counted.
    ///
    /// On success, will transform string input to a numeric type.
    Digits(usize),
    /// The field under validation, if present, must have a length between the given min and max.
    ///
    /// On success, will transform string input to a numeric type.
    DigitsBetween(usize, usize),
    /// When working with arrays, the field under validation must not have any duplicate values
    /// if it is present.
    Distinct,
    /// The field under validation, if present, must be formatted as an e-mail address.
    Email,
    #[cfg(feature = "pg")]
    /// The field under validation must exist in the given table.
    /// A column name may be specified; otherwise, the name of the field is used.
    Exists(&'a postgres::Connection, &'static str, Option<&'static str>),
    /// The field under validation must not be empty when it is present.
    Filled,
    /// The field under validation, if present, must be included in the given list of values.
    In(Vec<Value>),
    /// The field under validation, if present, must exist in `anotherfield`'s values.
    InArray(&'static str),
    /// The field under validation, if present, must be an integer.
    ///
    /// On success, will transform string input to a numeric type.
    Integer,
    /// The field under validation, if present, must be an IP address.
    ///
    /// Accepts both IPv4 and IPv6 addresses.
    IpAddress,
    /// The field under validation, if present, must be a valid JSON string.
    Json,
    /// The field under validation, if present, must be less than or equal to a maximum value.
    /// Strings, numerics, and files are evaluated in the same fashion as the `Size` rule.
    Max(isize),
    /// The field under validation, if present, must have a minimum value.
    /// Strings, numerics, and files are evaluated in the same fashion as the `Size` rule.
    Min(isize),
    /// The field under validation must not be included in the given list of values.
    NotIn(Vec<Value>),
    /// The field under validation must not exist in `anotherfield`'s values.
    NotInArray(&'static str),
    /// The field under validation, if present, must be numeric.
    ///
    /// On success, will transform string input to a numeric type.
    Numeric,
    /// The field under validation must be present in the input data but can be empty.
    Present,
    /// The field under validation, if present, must match the given regular expression.
    ///
    /// On success, will transform input to a string.
    Regex(&'static str),
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
    RequiredIf(&'static str, Value),
    /// The field under validation must be present unless the `anotherfield` field
    /// is equal to any `value`.
    RequiredUnless(&'static str, Value),
    /// The field under validation must be present only if
    /// any of the other specified fields are present.
    RequiredWith(Vec<&'static str>),
    /// The field under validation must be present only if
    /// all of the other specified fields are present.
    RequiredWithAll(Vec<&'static str>),
    /// The field under validation must be present only when
    /// any of the other specified fields are not present.
    RequiredWithout(Vec<&'static str>),
    /// The field under validation must be present only when
    /// all of the other specified fields are not present.
    RequiredWithoutAll(Vec<&'static str>),
    /// The given field must match the field under validation.
    Same(&'static str),
    /// The field under validation must have a size matching the given value.
    ///
    /// For string data, value corresponds to the number of characters.
    ///
    /// For numeric data, value corresponds to a given integer value.
    ///
    /// For files, size corresponds to the file size in kilobytes.
    Size(isize),
    /// The field under validation, if present, must be a string.
    String,
    #[cfg(feature = "pg")]
    /// The field under validation must not exist in the given table.
    /// A column name may be specified; otherwise, the name of the field is used.
    Unique(&'a postgres::Connection, &'static str, Option<&'static str>),
    /// The field under validation, if present, must be formatted as a valid URL,
    /// but does not need to resolve to a real website. The URL must contain the scheme
    /// or else it will fail validation. For example, `http://google.com` will
    /// pass validation, but `google.com` will fail validation.
    Url,
    #[doc(hidden)]
    #[cfg(not(feature = "pg"))]
    Phantom(&'a std::marker::PhantomData<u8>),
}

/// Validate a map of `values` against a map of `rules`.
///
/// Returns a `Result` containing a map of post-processed `values`,
/// or a map of validation error messages.
pub fn validate(rules: &BTreeMap<&'static str, Vec<Rule>>,
                values: Map)
                -> Result<Map, BTreeMap<&'static str, Vec<String>>> {
    let mut new_values = values;
    let mut errors = BTreeMap::new();

    for (field, ruleset) in rules {
        let array_field = field.split('.').collect::<Vec<&str>>();
        let mut current_errors = Vec::new();
        for rule in ruleset {
            let result = match *rule {
                Rule::Accepted => {
                    validators::accepted::validate_accepted(&new_values, &array_field)
                }
                Rule::ActiveUrl => {
                    validators::active_url::validate_active_url(&new_values, &array_field)
                }
                Rule::Alpha => validators::alpha::validate_alpha(&new_values, &array_field),
                Rule::AlphaDash => {
                    validators::alpha_dash::validate_alpha_dash(&new_values, &array_field)
                }
                Rule::AlphaNumeric => {
                    validators::alpha_numeric::validate_alpha_numeric(&new_values, &array_field)
                }
                Rule::Array => validators::array::validate_array(&new_values, &array_field),
                Rule::Between(min, max) => {
                    validators::between::validate_between(&new_values, &array_field, min, max)
                }
                Rule::Boolean => validators::boolean::validate_boolean(&new_values, &array_field),
                Rule::Confirmed => {
                    validators::confirmed::validate_confirmed(&new_values, &array_field)
                }
                Rule::Different(other) => {
                    let other = other.split('.').collect::<Vec<&str>>();
                    validators::different::validate_different(&new_values, &array_field, &other)
                }
                Rule::Digits(digits) => {
                    validators::digits::validate_digits(&new_values, &array_field, digits)
                }
                Rule::DigitsBetween(min, max) => {
                    validators::digits_between::validate_digits_between(&new_values,
                                                                        &array_field,
                                                                        min,
                                                                        max)
                }
                Rule::Distinct => {
                    validators::distinct::validate_distinct(&new_values, &array_field)
                }
                Rule::Email => validators::email::validate_email(&new_values, &array_field),
                #[cfg(feature = "pg")]
                Rule::Exists(conn, table, column) => {
                    validators::exists::validate_exists(conn,
                                                        &new_values,
                                                        &array_field,
                                                        table,
                                                        column)
                }
                Rule::Filled => validators::filled::validate_filled(&new_values, &array_field),
                Rule::In(ref options) => {
                    validators::in_const::validate_in(&new_values, &array_field, options)
                }
                Rule::InArray(other) => {
                    let other = other.split('.').collect::<Vec<&str>>();
                    validators::in_array::validate_in_array(&new_values, &array_field, &other)
                }
                Rule::Integer => validators::integer::validate_integer(&new_values, &array_field),
                Rule::IpAddress => {
                    validators::ip_address::validate_ip_address(&new_values, &array_field)
                }
                Rule::Json => validators::json::validate_json(&new_values, &array_field),
                Rule::Max(target) => {
                    validators::max::validate_max(&new_values, &array_field, target)
                }
                Rule::Min(target) => {
                    validators::min::validate_min(&new_values, &array_field, target)
                }
                Rule::NotIn(ref options) => {
                    validators::not_in::validate_not_in(&new_values, &array_field, options)
                }
                Rule::NotInArray(other) => {
                    let other = other.split('.').collect::<Vec<&str>>();
                    validators::not_in_array::validate_not_in_array(&new_values,
                                                                    &array_field,
                                                                    &other)
                }
                Rule::Numeric => validators::numeric::validate_numeric(&new_values, &array_field),
                Rule::Present => validators::present::validate_present(&new_values, &array_field),
                Rule::Regex(pattern) => {
                    validators::regex::validate_regex(&new_values, &array_field, pattern)
                }
                Rule::Required => {
                    validators::required::validate_required(&new_values, &array_field)
                }
                Rule::RequiredIf(other, ref condition) => {
                    let other = other.split('.').collect::<Vec<&str>>();
                    validators::required_if::validate_required_if(&new_values,
                                                                  &array_field,
                                                                  &other,
                                                                  condition)
                }
                Rule::RequiredUnless(other, ref condition) => {
                    let other = other.split('.').collect::<Vec<&str>>();
                    validators::required_unless::validate_required_unless(&new_values,
                                                                          &array_field,
                                                                          &other,
                                                                          condition)
                }
                Rule::RequiredWith(ref others) => {
                    let others = others.iter()
                        .map(|o| o.split('.').collect::<Vec<&str>>())
                        .collect::<Vec<Vec<&str>>>();
                    validators::required_with::validate_required_with(&new_values,
                                                                      &array_field,
                                                                      &others)
                }
                Rule::RequiredWithAll(ref others) => {
                    let others = others.iter()
                        .map(|o| o.split('.').collect::<Vec<&str>>())
                        .collect::<Vec<Vec<&str>>>();
                    validators::required_with_all::validate_required_with_all(&new_values,
                                                                              &array_field,
                                                                              &others)
                }
                Rule::RequiredWithout(ref others) => {
                    let others = others.iter()
                        .map(|o| o.split('.').collect::<Vec<&str>>())
                        .collect::<Vec<Vec<&str>>>();
                    validators::required_without::validate_required_without(&new_values,
                                                                            &array_field,
                                                                            &others)
                }
                Rule::RequiredWithoutAll(ref others) => {
                    let others = others.iter()
                        .map(|o| o.split('.').collect::<Vec<&str>>())
                        .collect::<Vec<Vec<&str>>>();
                    validators::required_without_all::validate_required_without_all(&new_values,
                                                                                    &array_field,
                                                                                    &others)
                }
                Rule::Same(other) => {
                    let other = other.split('.').collect::<Vec<&str>>();
                    validators::same::validate_same(&new_values, &array_field, &other)
                }
                Rule::Size(target) => {
                    validators::size::validate_size(&new_values, &array_field, target)
                }
                Rule::String => validators::string::validate_string(&new_values, &array_field),
                #[cfg(feature = "pg")]
                Rule::Unique(conn, table, column) => {
                    validators::unique::validate_unique(conn,
                                                        &new_values,
                                                        &array_field,
                                                        table,
                                                        column)
                }
                Rule::Url => validators::url::validate_url(&new_values, &array_field),
                #[cfg(not(feature = "pg"))]
                Rule::Phantom(_) => unimplemented!(),
            };
            match result {
                Ok(Some(res)) => {
                    new_values.assign(&array_field.iter()
                                           .enumerate()
                                           .map(|(i, x)| if i == 0 {
                                                    x.to_string()
                                                } else {
                                                    format!("[{}]", x)
                                                })
                                           .collect::<String>(),
                                      res)
                        .ok();
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
