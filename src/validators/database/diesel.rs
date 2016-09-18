use diesel::Connection;
use diesel::expression::sql_literal::sql;
use diesel::pg::PgConnection;
use diesel::query_builder::{BindCollector, QueryFragment};
use diesel::query_builder::bind_collector::RawBytesBindCollector;
use diesel::types::{Text, ToSql};

use params::{Map, Value};

use validators::database::DbConnection;

impl DbConnection for PgConnection {
    fn validate_exists(&self,
                       values: &Map,
                       field: &str,
                       table: &str,
                       column: Option<&str>)
                       -> Result<Option<Value>, String> {
        let column = if let Some(c) = column { c } else { field };
        let query = format!("SELECT EXISTS(SELECT 1 FROM {} WHERE {} = $1 LIMIT 1)",
                            table,
                            column);
        let mut bind: Vec<u8> = Vec::new();
        match values.find(&[field]) {
            Some(&Value::String(value)) => value.to_sql(&mut bind),
            Some(&Value::U64(value)) => (value as i64).to_sql(&mut bind),
            Some(&Value::I64(value)) => value.to_sql(&mut bind),
            Some(&Value::F64(value)) => value.to_sql(&mut bind),
            Some(&Value::Boolean(value)) => value.to_sql(&mut bind),
            None => "".to_sql(&mut bind),
            _ => {
                return Err(format!("The {} field must exist in the database.",
                                   field.to_lowercase().replace("_", " ")));
            }
        };
        let query = sql(&query);

        // TODO: Bind the thing
        // TODO: Run the query
        let mut bind_collector = RawBytesBindCollector::new();
        bind_collector.push_bound_value(Some(bind));
        let exists: bool = *self.query_all(query).unwrap().get(0).unwrap();
        if exists {
            Ok(None)
        } else {
            Err(format!("The {} field must exist in the database.",
                        field.to_lowercase().replace("_", " ")))
        }
    }

    fn validate_unique(&self,
                       values: &Map,
                       field: &str,
                       table: &str,
                       column: Option<&str>)
                       -> Result<Option<Value>, String> {
        unimplemented!()
    }
}
