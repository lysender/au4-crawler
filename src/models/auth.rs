use serde::{Deserialize, Serialize};

use super::user::User;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AuthSuccess {
    pub user: Option<User>,
    pub token: Option<String>,
    pub project_id: Option<String>,
    pub mfa_required: Option<bool>,
    pub mfa_validated: Option<bool>,
    pub mfa_auth_token: Option<String>,
}

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AuthPayload {
    pub username: String,
    pub password: String,
    pub captcha_token: String,
}
