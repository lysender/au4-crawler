use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Account {
    pub id: String,
    pub tier: String,
    pub r#type: String,
    pub user_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AccountPartial {
    pub id: Option<String>,
    pub tier: Option<String>,
    pub r#type: Option<String>,
    pub user_id: Option<String>,
}
