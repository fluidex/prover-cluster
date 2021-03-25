use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct Task {
    pub id: String,
}
