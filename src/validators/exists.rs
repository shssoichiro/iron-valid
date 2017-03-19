use postgres::Connection;
use postgres::types::ToSql;

use params::{Map, Value};

pub fn validate_exists(conn: &Connection,
                       values: &Map,
                       field: &[&str],
                       table: &str,
                       column: Option<&str>)
                       -> Result<Option<Value>, String> {
    let column = if let Some(c) = column {
        c
    } else {
        field.last().unwrap()
    };
    let query = format!("SELECT COUNT({}) as c FROM {} WHERE {} = $1",
                        column,
                        table,
                        column);
    let result = match values.find(field) {
        Some(&Value::String(ref value)) => conn.query(&query, vec![value as &ToSql].as_slice()),
        Some(&Value::U64(ref value)) => {
            conn.query(&query, vec![&(*value as i64) as &ToSql].as_slice())
        }
        Some(&Value::I64(ref value)) => conn.query(&query, vec![value as &ToSql].as_slice()),
        Some(&Value::F64(ref value)) => conn.query(&query, vec![value as &ToSql].as_slice()),
        Some(&Value::Boolean(ref value)) => conn.query(&query, vec![value as &ToSql].as_slice()),
        None => conn.query(&query, vec![&"" as &ToSql].as_slice()),
        _ => {
            return Err(format!("The {} field must exist in the database.",
                               field.last()
                                   .unwrap()
                                   .to_lowercase()
                                   .replace("_", " ")));
        }
    };

    let count: i64 = result.unwrap().get(0).get(0);
    if count > 0 {
        Ok(None)
    } else {
        Err(format!("The {} field must exist in the database.",
                    field.last()
                        .unwrap()
                        .to_lowercase()
                        .replace("_", " ")))
    }
}
