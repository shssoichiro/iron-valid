#[cfg(feature = "diesel")]
pub mod diesel;

#[cfg(feature = "postgres")]
pub mod postgres;

use params::{Map, Value};

pub trait DbConnection {
    fn validate_exists(&self,
                       values: &Map,
                       field: &str,
                       table: &str,
                       column: Option<&str>)
                       -> Result<Option<Value>, String>;
    fn validate_unique(&self,
                       values: &Map,
                       field: &str,
                       table: &str,
                       column: Option<&str>)
                       -> Result<Option<Value>, String>;
}
