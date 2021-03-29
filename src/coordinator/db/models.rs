use serde::Serialize;

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

#[derive(sqlx::FromRow, Debug, Clone, Serialize)]
pub struct Task {
    pub id: i64,
    pub task_id: String,
    // pub circuit: types::OrderType,
    pub witness: String,
    pub proof: String,
    pub status: TaskStatus,
    pub prover_id: String,
    pub created_time: TimestampDbType,
    pub updated_time: TimestampDbType,
}
