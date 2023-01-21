use std::path::Path;

use anyhow::Result;
use heed::types::{OwnedType, SerdeJson};
use heed::{Database, Env, EnvOpenOptions, RoTxn};
use roaring::RoaringBitmap;

use crate::codec::RoaringBitmapCodec;
use crate::{Query, Status, Task, TaskId, Type};

pub struct CustomStore {
    env: Env,
    tasks: Database<OwnedType<TaskId>, SerdeJson<Task>>,
    statuses: Database<SerdeJson<Status>, RoaringBitmapCodec>,
    types: Database<SerdeJson<Type>, RoaringBitmapCodec>,
}

impl CustomStore {
    pub fn new() -> Self {
        let db_path = Path::new("store.custom");
        std::fs::create_dir_all(db_path).unwrap();
        let env = EnvOpenOptions::new().max_dbs(4).open(db_path).unwrap();
        let mut wtxn = env.write_txn().unwrap();

        let tasks = env.create_database(&mut wtxn, Some("tasks")).unwrap();
        let statuses = env.create_database(&mut wtxn, Some("statuses")).unwrap();
        let types = env.create_database(&mut wtxn, Some("types")).unwrap();

        wtxn.commit().unwrap();

        Self {
            env,
            tasks,
            statuses,
            types,
        }
    }

    fn last_task_id(&self, rtxn: &RoTxn) -> Result<TaskId> {
        Ok(self.tasks.last(rtxn)?.map_or(0, |(k, _v)| k))
    }

    // if you call this function twice at the same time the second call will wait on the blocking `write_txn`.
    // reading while there is an insertion isn't an issue though.
    pub fn insert(&self, task: &Task) -> Result<()> {
        let mut wtxn = self.env.write_txn()?;

        self.tasks.put(&mut wtxn, &task.id, &task)?;

        let mut statuses = self
            .statuses
            .get(&mut wtxn, &task.status)?
            .unwrap_or_default();
        statuses.insert(1);
        self.statuses.put(&mut wtxn, &task.status, &statuses)?;

        let mut types = self.types.get(&mut wtxn, &task.r#type)?.unwrap_or_default();
        types.insert(1);
        self.types.put(&mut wtxn, &task.r#type, &types)?;

        Ok(())
    }

    pub fn query(&self, query: &Query) -> Result<Vec<Task>> {
        let rtxn = self.env.read_txn()?;
        let mut tasks = match query.task_id {
            None => RoaringBitmap::from_sorted_iter(0..self.last_task_id(&rtxn)?)?,
            Some(ref ids) => ids.iter().copied().collect(),
        };

        if let Some(ref statuses) = query.statuses {
            let mut task_status = RoaringBitmap::new();
            for status in statuses {
                if let Some(status) = self.statuses.get(&rtxn, &status)? {
                    task_status |= status;
                }
            }
            tasks &= task_status;
        }

        if let Some(ref types) = query.types {
            let mut task_type = RoaringBitmap::new();
            for type_ in types {
                if let Some(type_) = self.types.get(&rtxn, &type_)? {
                    task_type |= type_;
                }
            }
            tasks &= task_type;
        }

        if let Some(after_id) = query.after_id {
            tasks.remove_range(after_id..);
        }

        if let Some(before_id) = query.before_id {
            tasks.remove_range(..before_id);
        }

        tasks
            .into_iter()
            .skip(query.offset)
            .map(|id| {
                self.tasks
                    .get(&rtxn, &id)
                    .map(|task| task.expect("Corrupted database"))
                    .map_err(|err| err.into())
            })
            .take(query.limit)
            .collect()
    }
}
