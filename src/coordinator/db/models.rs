use crate::pb;
use serde::{Deserialize, Serialize};

pub type TimestampDbType = chrono::NaiveDateTime;

pub mod tablenames {
    pub const TASK: &str = "task";
}

#[derive(sqlx::Type, Debug, Clone, Serialize)]
#[sqlx(type_name = "task_status", rename_all = "snake_case")]
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
            // _ => unreachable!(),
        }
    }
}

#[derive(sqlx::FromRow, Debug, Clone, Serialize)]
pub struct Task {
    // pub id: i64,
    pub task_id: String,
    pub circuit: CircuitType,
    pub witness: Vec<u8>,
    pub proof: Option<Vec<u8>>,
    pub status: TaskStatus,
    pub prover_id: Option<String>,
    pub created_time: TimestampDbType,
    pub updated_time: TimestampDbType,
}
