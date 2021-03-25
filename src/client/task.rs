use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, Copy, PartialEq)]
pub struct Task {
    pub id: String,
}
