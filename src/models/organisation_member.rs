use serde::{Deserialize, Serialize};

use super::{organisation::Organisation, user::User};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OrganisationMember {
    pub id: String,
    pub organisation: Option<Organisation>,
    pub user: Option<User>,
    pub email: Option<String>,
    pub groups: Vec<String>,
    pub status: String,
    pub is_unregistered_user: Option<bool>,
    pub invitation_token: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}
