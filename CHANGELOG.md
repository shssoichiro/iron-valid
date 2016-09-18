**Version 0.3.0 (unreleased)**
 - [SEMVER_MAJOR] Upgrade params to 0.5.x
 - [SEMVER_MAJOR] Rename "pg" feature to "postgres"
 - [SEMVER_MAJOR] Validators previously accepting `Vec` arguments now accept `&[]` arguments instead
 - [SEMVER_MINOR] Add "diesel" feature to enable database validators to work with diesel

**Version 0.2.0**
 - Add `exists` and `unique` database validators, available with `pg` feature enabled
 - Upgrade to iron 0.4.0 and params 0.3.0

**Version 0.1.0**
 - Initial release
