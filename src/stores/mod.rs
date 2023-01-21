use core::fmt;
use std::str::FromStr;

use serde::{Deserialize, Serialize};

pub mod custom;
pub mod sqlite;

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

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
pub struct Task {
    id: TaskId,
    status: Status,
    r#type: Type,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Query {
    task_id: Option<Vec<TaskId>>,

    after_id: Option<TaskId>,
    before_id: Option<TaskId>,

    statuses: Option<Vec<Status>>,
    types: Option<Vec<Type>>,

    limit: usize,
    offset: usize,
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
