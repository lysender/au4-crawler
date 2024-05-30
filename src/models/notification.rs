use serde::{Deserialize, Serialize};

use super::user::User;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Notification {
    pub id: String,
    pub r#type: String,
    pub project_id: Option<String>,
    pub aggregate: Option<String>,
    pub aggregate_id: Option<String>,
    pub event: Option<String>,
    pub last_event: Option<String>,
    pub actor: Option<User>,
    pub message: String,
    pub url: Option<String>,
    pub read: bool,
    pub read_at: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}
