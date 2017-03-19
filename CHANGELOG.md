**Version 0.5.0**
 - [SEMVER_MAJOR] `validate` now takes `rules` by reference
 - [SEMVER_MINOR] Nested fields can be validated. For example, to validate
 a struct `{ user: { name: 'foo', email: 'a@b.com' } }`, you could pass in `user.name`
 or `user.email` as the key on the `rules` passed in to `validate`. This also
 works for `Rule`s that accept a parameter for a second field. ([#34](https://github.com/shssoichiro/iron-valid/issues/34))

**Version 0.4.1**
 - Allow postgres 0.14.0 (previous versions still work with this crate)

**Version 0.4.0**
 - Bump params to 0.6.0
 - Bump regex to 0.2.0
 - Switch to serde_json for testing if json is valid
 - Remove any explicit minimum supported Rust version 
    (this should always work on the latest stable,
    and may work on earlier versions but is not guaranteed to)

**Version 0.3.1**
 - Bump compatible postgres versions to include up to 0.13.x

**Version 0.3.0**
 - [SEMVER_MAJOR] Upgrade params to 0.5.x
 - [SEMVER_MAJOR] Bump minimum rustc version to 1.9.0
 - Code cleanup

**Version 0.2.0**
 - Add `exists` and `unique` database validators, available with `pg` feature enabled
 - Upgrade to iron 0.4.0 and params 0.3.0

**Version 0.1.0**
 - Initial release
