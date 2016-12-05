# iron_valid

[![Build Status](https://travis-ci.org/shssoichiro/iron-valid.svg?branch=master)](https://travis-ci.org/shssoichiro/iron-valid)
[![Version](https://img.shields.io/crates/v/iron-valid.svg)](https://crates.io/crates/iron-valid)
[![License](https://img.shields.io/crates/l/iron-valid.svg)](https://github.com/shssoichiro/iron-valid/blob/master/LICENSE)

## Overview

iron_valid is a form validation library for iron that integrates with the [params](https://crates.io/crates/params) crate. It is based on [Laravel](https://laravel.com/docs/5.2/validation)'s validation, and works in a similar way.

If you encounter an issue, please report it via the GitHub issues tab. Include as many details
as possible.

## Usage

An example of using iron_valid to validate a form request is as follows:

```rust
use iron::prelude::*;
use iron::status;
use iron_valid::{Rule, validate};
use params::{Params, Value};
use std::collections::BTreeMap;

pub fn register(req: &mut Request) -> IronResult<Response> {
    let params = match req.get::<Params>() {
        Ok(p) => p,
        Err(_) => {
            return Ok(Response::with((status::BadRequest)));
        }
    };

    let mut rules = BTreeMap::new();
    rules.insert("email", vec![Rule::Required, Rule::Email]);
    rules.insert("password", vec![Rule::Required, Rule::Confirmed, Rule::Min(8)]);

    match validate(rules, params) {
        Ok(ref values) => {
            // Save the user to the database
            let email = values.find(&["email"]).unwrap();

            // ...

            Ok(Response::with((status::Ok)))
        }
        Err(_) => Ok(Response::with((status::BadRequest))),
    }
}
```

This example would parse the request parameters using the params crate, then validate the `email` and `password` fields from the request using the specified rules for each.

[Full documentation, along with a list of validation rules, is available here.](http://shssoichiro.github.io/iron-valid/iron_valid/)

The current minimum supported Rust version is **1.9.0**.

iron_valid follows Semantic Versioning.

## Contributing

Any contributions are welcome and will be accepted via pull request on GitHub. Bug reports can be
filed via GitHub issues. If you have the capability to submit a fix with the bug report, it is
preferred that you do so via pull request, however you do not need to be a Rust programmer to
contribute. Other contributions (such as improving documentation or translations) are also
welcome via GitHub.

## License

iron_valid is open-source software, distributed under the MIT license.
