use serde::{Deserialize, Serialize};

use super::actor::Actor;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ActivityLog {
    pub actor: Option<Actor>,
    pub aggregate: String,
    pub aggregate_id: String,
    pub organisation_id: Option<String>,
    pub project_id: Option<String>,
    pub issue_id: Option<String>,
    pub created_at: String,
    pub event: String,
    pub id: String,
    pub message: String,
    pub url: Option<String>,
}
