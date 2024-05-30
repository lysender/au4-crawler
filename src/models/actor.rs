use serde::{Deserialize, Serialize};

use super::{account::Account, user::User};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Actor {
    pub id: Option<String>,
    pub user: Option<User>,
    pub accounts: Option<Vec<Account>>,
    pub r#type: Option<String>,
    pub user_roles: Option<Vec<String>>,
}
