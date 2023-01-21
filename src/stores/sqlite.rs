use std::fmt::Write;
use std::str::FromStr;

use anyhow::Result;
use rusqlite::types::{FromSql, FromSqlError, FromSqlResult, ToSqlOutput, ValueRef};
use rusqlite::{params, Connection, ToSql};

use crate::{Query, Status, Task, Type};

pub struct SqliteStore {
    com: Connection,
}

impl SqliteStore {
    pub fn new() -> Self {
        let connection = Connection::open("store.sqlite").unwrap();
        connection
            .prepare(
                r#"
                CREATE TABLE IF NOT EXISTS tasks (
                    task_id INT PRIMARY KEY,
                    status TEXT,
                    type TEXT
                );
                "#,
            )
            .expect("Error while preparing init query")
            .raw_execute()
            .expect("Error while executing init query");

        SqliteStore { com: connection }
    }

    pub fn insert(&self, task: &Task) -> Result<()> {
        self.com
            .prepare(
                r#"
                INSERT INTO tasks (task_id, status, type) VALUES (?, ?, ?);
                "#,
            )?
            .execute(params![task.id, task.status, task.r#type])?;

        Ok(())
    }

    pub fn query(&self, query: &Query) -> Result<Vec<Task>> {
        let mut request = format!(
            "SELECT task_id, status, type FROM tasks LIMIT {} OFFSET {} WHERE",
            query.limit, query.offset
        );

        if let Some(ref task_ids) = query.task_id {
            write!(
                request,
                " task_id IN [{}]",
                task_ids
                    .iter()
                    .map(|id| id.to_string())
                    .collect::<Vec<_>>()
                    .join(", ")
            )?;
        }

        if let Some(after_id) = query.after_id {
            write!(request, " task_id > {after_id}")?;
        }

        if let Some(before_id) = query.before_id {
            write!(request, " task_id < {before_id}")?;
        }

        if let Some(ref statuses) = query.statuses {
            write!(
                request,
                " status IN [{}]",
                statuses
                    .iter()
                    .map(|id| id.to_string())
                    .collect::<Vec<_>>()
                    .join(", ")
            )?;
        }

        if let Some(ref types) = query.types {
            write!(
                request,
                " type IN [{}]",
                types
                    .iter()
                    .map(|id| id.to_string())
                    .collect::<Vec<_>>()
                    .join(", ")
            )?;
        }

        if request.ends_with("WHERE") {
            request.replace_range(request.len() - "WHERE".len()..request.len(), ";");
        } else {
            request.push_str(";");
        }

        Ok(self
            .com
            .prepare(&request)?
            .query_map(params![], |row| {
                Ok(Task {
                    id: row.get(0)?,
                    status: row.get(1)?,
                    r#type: row.get(2)?,
                })
            })?
            .collect::<Result<_, _>>()?)
    }
}

impl ToSql for Status {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        Ok(self.to_string().into())
    }
}

impl FromSql for Status {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        match value {
            rusqlite::types::ValueRef::Text(s) => {
                Ok(Self::from_str(std::str::from_utf8(s).unwrap()).unwrap()) // .map_err(|_| FromSqlError::Other)
            }
            rusqlite::types::ValueRef::Integer(_)
            | rusqlite::types::ValueRef::Real(_)
            | rusqlite::types::ValueRef::Blob(_)
            | rusqlite::types::ValueRef::Null => Err(FromSqlError::InvalidType),
        }
    }
}

impl ToSql for Type {
    fn to_sql(&self) -> rusqlite::Result<rusqlite::types::ToSqlOutput<'_>> {
        Ok(self.to_string().into())
    }
}

impl FromSql for Type {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        match value {
            rusqlite::types::ValueRef::Text(s) => {
                Ok(Self::from_str(std::str::from_utf8(s).unwrap()).unwrap()) //.map_err(|_| FromSqlError::Other)
            }
            rusqlite::types::ValueRef::Integer(_)
            | rusqlite::types::ValueRef::Real(_)
            | rusqlite::types::ValueRef::Blob(_)
            | rusqlite::types::ValueRef::Null => Err(FromSqlError::InvalidType),
        }
    }
}
