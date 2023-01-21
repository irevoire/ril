use clap::Parser;

use crate::{Query, Task};

#[derive(Debug, Parser)]
pub enum Option {
    Insert(Task),
    Query(Query),
}
