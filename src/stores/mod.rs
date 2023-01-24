use anyhow::Result;

mod custom;
mod sqlite;

pub use custom::CustomStore;
pub use sqlite::SqliteStore;

use crate::{Query, Task};

pub trait Store {
    fn insert(&self, task: &Task) -> Result<()>;
    fn query(&self, query: &Query) -> Result<Vec<Task>>;
    fn delete(self) -> Result<()>;
}
