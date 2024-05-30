use serde::{Deserialize, Serialize};

use super::user::User;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct File {
    pub id: String,
    pub category: String,
    pub user: Option<User>,
    pub project_id: Option<String>,
    pub account_id: Option<String>,
    pub labels: Vec<String>,
    pub content_type: String,
    pub name: String,
    pub filename: String,
    pub path: String,
    pub size: i64,
    pub url: String,
    pub is_image: bool,
    pub is_public: bool,
    pub versions: Option<Vec<FileVersion>>,
    pub state: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FileVersion {
    pub name: String,
    pub path: String,
    pub url: String,
}
