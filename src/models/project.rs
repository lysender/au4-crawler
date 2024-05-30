use serde::{Deserialize, Serialize};

use super::issue_status::IssueStatus;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Project {
    pub id: String,
    pub key: String,
    pub name: String,
    pub preferences: Option<ProjectPreferences>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ProjectPartial {
    pub id: Option<String>,
    pub key: Option<String>,
    pub name: Option<String>,
    pub preferences: Option<ProjectPreferences>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ProjectSlim {
    pub id: String,
    pub key: String,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ProjectPreferences {
    pub issue_statuses: Vec<IssueStatus>,
    pub issue_type: String,
    pub estimate_type: String,
}
