use serde::{Deserialize, Serialize};

use super::{account::AccountPartial, avatar::Avatar, user::UserPartial};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Organisation {
    pub id: String,
    pub tier: String,
    pub r#type: String,
    pub user_id: String,
    pub avatar: Option<Avatar>,
    pub owner: Option<UserPartial>,
    pub account: Option<AccountPartial>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OrganisationPartial {
    pub id: Option<String>,
    pub tier: Option<String>,
    pub r#type: Option<String>,
    pub user_id: Option<String>,
    pub avatar: Option<Avatar>,
    pub owner: Option<UserPartial>,
    pub account: Option<AccountPartial>,
}
