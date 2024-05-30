use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IssueStatus {
    pub id: String,
    pub name: String,
}
