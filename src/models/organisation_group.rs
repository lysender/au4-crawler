use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OrganisationGroup {
    pub id: String,
    pub is_custom_group: bool,
    pub name: String,
    pub description: Option<String>,
    pub roles: Vec<String>,
    pub permissions: Option<Vec<String>>,
    pub is_org_wide: bool,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}
