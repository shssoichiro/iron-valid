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
