use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Comment {
    pub id: String,
    pub topic_id: String,
    pub body: String,
    pub body_data: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}
