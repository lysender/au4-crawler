use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Authz {
    pub groups: Vec<String>,
    pub id: String,
    pub permissions: Vec<String>,
    pub roles: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: String,
    pub username: String,
    pub email: Option<String>,
    pub status: String,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub avatar: Option<Avatar>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserPartial {
    pub id: Option<String>,
    pub username: Option<String>,
    pub email: Option<String>,
    pub status: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub avatar: Option<Avatar>,
}

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

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Actor {
    pub id: Option<String>,
    pub user: Option<User>,
    pub accounts: Option<Vec<Account>>,
    pub r#type: Option<String>,
    pub user_roles: Option<Vec<String>>,
}

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

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum UserPreferenceValue {
    Flag(bool),
    Stringy(String),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserPreference {
    pub id: String,
    pub value: UserPreferenceValue,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Project {
    pub id: String,
    pub key: String,
    pub name: String,
    pub preferences: Option<ProjectPreferences>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ProjectPartial {
    pub id: Option<String>,
    pub key: Option<String>,
    pub name: Option<String>,
    pub preferences: Option<ProjectPreferences>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ProjectSlim {
    pub id: String,
    pub key: String,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ProjectPreferences {
    pub issue_statuses: Vec<IssueStatus>,
    pub issue_type: String,
    pub estimate_type: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Label {
    pub id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IssueStatus {
    pub id: String,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Issue {
    pub id: String,
    pub key: String,
    pub project_id: String,
    pub epic_id: Option<String>,
    pub parent_id: Option<String>,
    pub r#type: String,
    pub title: String,
    pub description: Option<String>,
    pub estimate: Option<u32>,
    pub estimate_type: Option<String>,
    pub labels: Option<Vec<String>>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct IssuePartial {
    pub id: Option<String>,
    pub key: Option<String>,
    pub project_id: Option<String>,
    pub epic_id: Option<String>,
    pub parent_id: Option<String>,
    pub r#type: Option<String>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub estimate: Option<u32>,
    pub estimate_type: Option<String>,
    pub labels: Option<Vec<String>>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProjectMember {
    pub id: String,
    pub user: Option<User>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CreateIssueBody {
    pub r#type: String,
    pub initiative_id: Option<String>,
    pub epic_id: Option<String>,
    pub parent_id: Option<String>,
    pub assignee_id: Option<String>,
    pub title: String,
    pub description: Option<String>,
    pub estimate_type: Option<String>,
    pub estimate: Option<u32>,
    pub status: Option<String>,
    pub labels: Vec<String>,
}

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

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PaginationMeta {
    pub page: u32,
    pub per_page: u32,
    pub total_records: u32,
    pub total_pages: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PaginationResult<T> {
    pub meta: PaginationMeta,
    pub data: Vec<T>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct IssueTimelineItem {
    pub actor: Option<Actor>,
    pub aggregate: String,
    pub aggregate_id: String,
    pub created_at: String,
    pub data: IssueTimelineItemData,
    pub event: String,
    pub id: String,
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct IssueTimelineItemData {
    pub issue: Option<IssuePartial>,
    pub subtask: Option<IssuePartial>,
    pub epic: Option<IssuePartial>,
    pub parent: Option<IssuePartial>,
    pub comment: Option<Comment>,
    pub initiative: Option<IssuePartial>,
    pub priority: Option<String>,
    pub previous_priority: Option<String>,
    pub labels: Option<Vec<String>>,
    pub updated_at: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Repository {
    pub id: String,
    pub project_id: String,
    pub name: String,
    pub description: Option<String>,
    pub integration: Option<RepositoryIntegration>,
    pub private: bool,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RepositoryIntegration {
    pub node_id: Option<String>,
    pub default_branch: Option<String>,
    pub provider: Option<String>,
    pub url: Option<String>,
    pub owner: Option<RepositoryOwner>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RepositoryOwner {
    pub node_id: String,
    pub username: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChannelKey {
    pub cluster: String,
    pub key: String,
}
