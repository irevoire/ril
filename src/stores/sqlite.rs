use anyhow::Result;
use rusqlite::Connection;

use super::Task;

pub struct SqliteStore {
    com: Connection,
}

impl SqliteStore {
    pub fn new() -> Self {
        SqliteStore {
            com: Connection::open("store.sqlite").unwrap(),
        }
    }

    pub fn register(&self, task: Task) -> Result<()> {
        let mut doc_stmt = self
            .com
            .prepare(
                r#"
            INSERT INTO documents (doc_id, document) VALUES (?, ?)
            ON CONFLICT(doc_id) DO UPDATE SET document = excluded.document;
            "#,
            )
            .unwrap();

        Ok(())
    }
}
