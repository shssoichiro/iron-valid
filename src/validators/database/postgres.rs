use postgres::Connection;
use postgres::types::ToSql;

use params::{Map, Value};

use validators::database::DbConnection;

impl DbConnection for Connection {
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
        let result = match values.find(&[field]) {
            Some(&Value::String(ref value)) => self.query(&query, vec![value as &ToSql].as_slice()),
            Some(&Value::U64(ref value)) => {
                self.query(&query, vec![&(*value as i64) as &ToSql].as_slice())
            }
            Some(&Value::I64(ref value)) => self.query(&query, vec![value as &ToSql].as_slice()),
            Some(&Value::F64(ref value)) => self.query(&query, vec![value as &ToSql].as_slice()),
            Some(&Value::Boolean(ref value)) => {
                self.query(&query, vec![value as &ToSql].as_slice())
            }
            None => self.query(&query, vec![&"" as &ToSql].as_slice()),
            _ => {
                return Err(format!("The {} field must exist in the database.",
                                   field.to_lowercase().replace("_", " ")));
            }
        };

        let exists: bool = result.unwrap().get(0).get(0);
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
        let column = if let Some(c) = column { c } else { field };
        let query = format!("SELECT COUNT({}) as c FROM {} WHERE {} = $1",
                            column,
                            table,
                            column);
        let result = match values.find(&[field]) {
            Some(&Value::String(ref value)) => self.query(&query, vec![value as &ToSql].as_slice()),
            Some(&Value::U64(ref value)) => {
                self.query(&query, vec![&(*value as i64) as &ToSql].as_slice())
            }
            Some(&Value::I64(ref value)) => self.query(&query, vec![value as &ToSql].as_slice()),
            Some(&Value::F64(ref value)) => self.query(&query, vec![value as &ToSql].as_slice()),
            Some(&Value::Boolean(ref value)) => {
                self.query(&query, vec![value as &ToSql].as_slice())
            }
            None => self.query(&query, vec![&"" as &ToSql].as_slice()),
            _ => {
                return Err(format!("The {} field must be unique.",
                                   field.to_lowercase().replace("_", " ")));
            }
        };

        let count: i64 = result.unwrap().get(0).get(0);
        if count == 0 {
            Ok(None)
        } else {
            Err(format!("The {} field must be unique.",
                        field.to_lowercase().replace("_", " ")))
        }
    }
}
