use serde::{Deserialize, Serialize};

pub mod custom;

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

    statuses: Option<Vec<Status>>,
    types: Option<Vec<Type>>,

    after_id: Option<TaskId>,
    before_id: Option<TaskId>,

    limit: usize,
    offset: usize,
}
