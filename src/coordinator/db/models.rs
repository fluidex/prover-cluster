use super::sqlxextend;
use super::DbType;
use serde::{Deserialize, Serialize};

pub type TimestampDbType = chrono::NaiveDateTime;

pub mod tablenames {
    pub const TASK: &str = "task";
}
use tablenames::*;

#[derive(sqlx::Type, Debug, Clone, Serialize)]
#[sqlx(type_name = "task_status", rename_all = "lowercase")]
pub enum TaskStatus {
    NotAssigned,
    Assigned,
    Proved,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Copy, sqlx::Type)]
#[sqlx(type_name = "varchar")]
#[sqlx(rename_all = "lowercase")]
pub enum CircuitType {
    BLOCK,
}

#[derive(sqlx::FromRow, Debug, Clone, Serialize)]
pub struct Task {
    // pub id: i64,
    pub task_id: String,
    pub circuit: CircuitType,
    pub witness: String,
    pub proof: String,
    pub status: TaskStatus,
    pub prover_id: String,
    pub created_time: TimestampDbType,
    pub updated_time: TimestampDbType,
}

impl sqlxextend::TableSchemas for Task {
    fn table_name() -> &'static str {
        TASK
    }
    const ARGN: i32 = 8;
    // fn default_argsn() -> Vec<i32> {
    //     vec![1]
    // }
}

impl sqlxextend::BindQueryArg<'_, DbType> for Task {
    fn bind_args<'g, 'q: 'g>(&'q self, arg: &mut impl sqlx::Arguments<'g, Database = DbType>) {
        arg.add(&self.task_id);
        arg.add(self.circuit);
        arg.add(&self.witness);
        arg.add(&self.proof);
        arg.add(self.status);
        arg.add(&self.prover_id);
        arg.add(self.created_time);
        arg.add(self.updated_time);
    }
}

impl sqlxextend::SqlxAction<'_, sqlxextend::InsertTable, DbType> for Task {}
