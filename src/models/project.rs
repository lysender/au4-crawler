use rand::Rng;
use serde::{Deserialize, Serialize};

use super::issue_status::IssueStatus;

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
#[serde(rename_all = "camelCase")]
pub struct CreateProjectBody {
    pub key: String,
    pub name: String,
    pub description: Option<String>,
    pub issue_type: Option<String>,
    pub workflow_type: Option<String>,
    pub estimate_type: Option<String>,
    pub backlog_on_board: Option<bool>,
}

pub fn generate_project_key(title: &str) -> String {
    let mut keys: Vec<String> = Vec::new();
    let chunks: Vec<&str> = title.split_whitespace().collect();
    for chunk in chunks.into_iter() {
        // Get first char
        let c = chunk.chars().next();
        if let Some(c) = c {
            if c.is_ascii_alphabetic() {
                keys.push(c.to_ascii_uppercase().to_string());
            }
        }
    }

    // Add random numbers from 0-99
    let num: u32 = rand::rng().random_range(1..=99);
    keys.push(num.to_string());

    keys.join("")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_project_key() {
        let title = "Awesome Test Project";
        let key = generate_project_key(title);
        assert!(key.starts_with("ATP"));
        assert!(key.len() > 3);
    }
}
