mod codec;
pub mod options;
pub mod stores;

use core::fmt;
use std::str::FromStr;

use clap::Parser;
use serde::{Deserialize, Serialize};

pub type TaskId = u32;

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
pub enum Status {
    Enqueued,
    Processing,
    Succeeded,
    Failed,
}

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
pub enum Type {
    IndexCreation,
    IndexDeletion,
    IndexSwap,
    DocumentAddition,
    DocumentDeletion,
}

#[derive(Debug, Serialize, Deserialize, Copy, Clone, Parser)]
pub struct Task {
    pub id: TaskId,
    pub status: Status,
    pub r#type: Type,
}

const DEFAULT_QUERY_LIMIT: fn() -> usize = || 20;

#[derive(Debug, Serialize, Deserialize, Parser)]
pub struct Query {
    #[arg(long)]
    task_id: Option<Vec<TaskId>>,

    #[arg(long)]
    after_id: Option<TaskId>,
    #[arg(long)]
    before_id: Option<TaskId>,

    #[arg(long)]
    statuses: Option<Vec<Status>>,
    #[arg(long)]
    types: Option<Vec<Type>>,

    #[serde(default = "DEFAULT_QUERY_LIMIT")]
    #[arg(long, default_value_t = DEFAULT_QUERY_LIMIT())]
    limit: usize,
    #[serde(default)]
    #[arg(long, default_value_t = usize::default())]
    offset: usize,
}

impl Query {
    pub fn is_empty(&self) -> bool {
        self.task_id.is_none()
            && self.after_id.is_none()
            && self.before_id.is_none()
            && self.statuses.is_none()
            && self.types.is_none()
    }
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Status::Enqueued => write!(f, "enqueued"),
            Status::Processing => write!(f, "processing"),
            Status::Succeeded => write!(f, "succeeded"),
            Status::Failed => write!(f, "failed"),
        }
    }
}

impl FromStr for Status {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "enqueued" => Ok(Status::Enqueued),
            "processing" => Ok(Status::Processing),
            "succeeded" => Ok(Status::Succeeded),
            "failed" => Ok(Status::Failed),
            s => panic!("received unknow string while deserializing a Status: {s:?}."),
        }
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Type::IndexCreation => write!(f, "indexCreation"),
            Type::IndexDeletion => write!(f, "indexDeletion"),
            Type::IndexSwap => write!(f, "indexSwap"),
            Type::DocumentAddition => write!(f, "documentAddition"),
            Type::DocumentDeletion => write!(f, "documentDeletion"),
        }
    }
}

impl FromStr for Type {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "indexCreation" => Ok(Type::IndexCreation),
            "indexDeletion" => Ok(Type::IndexDeletion),
            "indexSwap" => Ok(Type::IndexSwap),
            "documentAddition" => Ok(Type::DocumentAddition),
            "documentDeletion" => Ok(Type::DocumentDeletion),
            s => panic!("received unknow string while deserializing a Type: {s:?}."),
        }
    }
}
