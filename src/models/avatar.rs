use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Avatar {
    pub id: Option<String>,
    pub url: String,
    pub versions: Option<AvatarVersions>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AvatarVersions {
    pub x: Option<String>,
    pub xs: Option<String>,
}
