use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Issue {
    pub id: String,
    pub key: String,
    pub project_id: String,
    pub epic_id: Option<String>,
    pub parent_id: Option<String>,
    pub r#type: String,
    pub title: String,
    pub description: Option<String>,
    pub estimate: Option<u32>,
    pub estimate_type: Option<String>,
    pub labels: Option<Vec<String>>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct IssuePartial {
    pub id: Option<String>,
    pub key: Option<String>,
    pub project_id: Option<String>,
    pub epic_id: Option<String>,
    pub parent_id: Option<String>,
    pub r#type: Option<String>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub estimate: Option<u32>,
    pub estimate_type: Option<String>,
    pub labels: Option<Vec<String>>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CreateIssueBody {
    pub r#type: String,
    pub initiative_id: Option<String>,
    pub epic_id: Option<String>,
    pub parent_id: Option<String>,
    pub assignee_id: Option<String>,
    pub title: String,
    pub description: Option<String>,
    pub estimate_type: Option<String>,
    pub estimate: Option<u32>,
    pub status: Option<String>,
    pub labels: Vec<String>,
}
