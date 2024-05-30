use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Repository {
    pub id: String,
    pub project_id: String,
    pub name: String,
    pub description: Option<String>,
    pub integration: Option<RepositoryIntegration>,
    pub private: bool,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RepositoryIntegration {
    pub node_id: Option<String>,
    pub default_branch: Option<String>,
    pub provider: Option<String>,
    pub url: Option<String>,
    pub owner: Option<RepositoryOwner>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RepositoryOwner {
    pub node_id: String,
    pub username: String,
}
