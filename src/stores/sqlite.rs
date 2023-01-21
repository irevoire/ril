use anyhow::Result;
use rusqlite::{
    params,
    types::{FromSql, FromSqlError, FromSqlResult, ToSqlOutput, ValueRef},
    Connection, ToSql,
};
use std::fmt::Write;
use std::str::FromStr;

use super::{Query, Status, Task, Type};

pub struct SqliteStore {
    com: Connection,
}

impl SqliteStore {
    pub fn new() -> Self {
        let connection = Connection::open("store.sqlite").unwrap();
        connection
            .prepare(
                r#"
                DROP TABLE IF EXISTS tasks;
                DROP TYPE IF EXISTS status;
                DROP TYPE IF EXISTS type;

                CREATE TYPE status AS ENUM ('enqueued', 'processing', 'succeeded', 'failed');
                CREATE TYPE type AS ENUM ('indexCreation', 'indexDeletion', 'indexSwap', 'documentAddition', 'documentDeletion');
                
                CREATE TABLE IF NOT EXISTS tasks (
                    task_id INT PRIMARY KEY,
                    status status,
                    type type
        );
        "#,
            )
            .expect("Error while preparing init query")
            .raw_execute()
            .expect("Error while executing init query");

        SqliteStore { com: connection }
    }

    pub fn register(&self, task: Task) -> Result<()> {
        self.com
            .prepare(
                r#"
            INSERT INTO tasks (task_id, status, type) VALUES (?, ?, ?);
            "#,
            )
            .unwrap()
            .execute(params![task.id, task.status, task.r#type])
            .expect("Error while inserting document");

        Ok(())
    }

    pub fn query(&self, query: Query) -> Result<Vec<Task>> {
        let mut request = format!(
            "SELECT (task_id, status, type) FROM tasks LIMIT {} OFFSET {} WHERE",
            query.limit, query.offset
        );

        if let Some(task_ids) = query.task_id {
            write!(
                request,
                " task_id IN [{}]",
                task_ids
                    .iter()
                    .map(|id| id.to_string())
                    .collect::<Vec<_>>()
                    .join(", ")
            )
            .unwrap();
        }

        if let Some(after_id) = query.after_id {
            write!(request, " task_id > {after_id}").unwrap();
        }

        if let Some(before_id) = query.before_id {
            write!(request, " task_id < {before_id}").unwrap();
        }

        if let Some(statuses) = query.statuses {
            write!(
                request,
                " status IN [{}]",
                statuses
                    .iter()
                    .map(|id| id.to_string())
                    .collect::<Vec<_>>()
                    .join(", ")
            )
            .unwrap();
        }

        if let Some(types) = query.types {
            write!(
                request,
                " type IN [{}]",
                types
                    .iter()
                    .map(|id| id.to_string())
                    .collect::<Vec<_>>()
                    .join(", ")
            )
            .unwrap();
        }

        request.push_str(";");

        Ok(self
            .com
            .prepare(&request)
            .unwrap()
            .query_map(params![], |row| {
                Ok(Task {
                    id: row.get(0)?,
                    status: row.get(1)?,
                    r#type: row.get(2)?,
                })
            })
            .unwrap()
            .map(Result::unwrap)
            .collect())
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
        Ok(serde_json::to_string(&self).unwrap().into())
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
