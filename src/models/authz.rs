use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Authz {
    pub groups: Vec<String>,
    pub id: String,
    pub permissions: Vec<String>,
    pub roles: Vec<String>,
}
