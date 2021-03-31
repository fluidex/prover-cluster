use crate::pb;
use serde::{Deserialize, Serialize};

pub type TimestampDbType = chrono::NaiveDateTime;

pub mod tablenames {
    pub const TASK: &str = "task";
}

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

impl From<pb::Circuit> for CircuitType {
    fn from(pb_circuit: pb::Circuit) -> Self {
        match pb_circuit {
            pb::Circuit::Block => Self::BLOCK,
            _ => unreachable!(),
        }
    }
}

impl CircuitType {
    pub fn to_db_string(&self) -> String {
        match self {
            Self::BLOCK => String::from("block"),
            _ => unreachable!(),
        }
    }
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
