use serde::{Deserialize, Serialize};

use super::user::User;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProjectMember {
    pub id: String,
    pub user: Option<User>,
}
