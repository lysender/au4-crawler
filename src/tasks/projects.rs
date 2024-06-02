use anyhow::anyhow;
use reqwest::Client;

use crate::{
    error::Result,
    models::{
        auth::AuthContext, issue_status::IssueStatus, label::Label, pagination::PaginationResult,
        project::Project,
    },
};

use super::{JSON_CONTENT_TYPE, USER_AGENT};

pub async fn fetch_projects(
    ctx: &AuthContext,
    page: u32,
    per_page: u32,
) -> Result<PaginationResult<Project>> {
    let url = format!("{}/projects", ctx.api_url.as_str());
    let query_params = vec![
        ("status", "active".to_string()),
        ("page", page.to_string()),
        ("per_page", per_page.to_string()),
        ("sort", "-lastActivityDate".to_string()),
        (
            "include",
            "meta,activeSprint,members,organisation".to_string(),
        ),
    ];

    let response = Client::new()
        .get(url)
        .query(&query_params)
        .header(reqwest::header::USER_AGENT, USER_AGENT)
        .header(reqwest::header::CONTENT_TYPE, JSON_CONTENT_TYPE)
        .bearer_auth(ctx.token.as_str())
        .send()
        .await?;

    if response.status().is_success() {
        let result: PaginationResult<Project> = response.json().await?;
        Ok(result)
    } else {
        let message = format!(
            "Unable to fetch project listing. Error: {}",
            response.status()
        );
        eprintln!("{}", message);
        Err(anyhow!(message))
    }
}

pub async fn fetch_project(ctx: &AuthContext, project_id: &str) -> Result<Project> {
    let url = format!("{}/projects/{}", ctx.api_url.as_str(), project_id);
    let query_params = vec![("include", "organisation".to_string())];
    let response = Client::new()
        .get(url)
        .query(&query_params)
        .header(reqwest::header::USER_AGENT, USER_AGENT)
        .header(reqwest::header::CONTENT_TYPE, JSON_CONTENT_TYPE)
        .bearer_auth(ctx.token.as_str())
        .send()
        .await?;

    if response.status().is_success() {
        let project: Project = response.json().await?;
        Ok(project)
    } else {
        Err(anyhow!(
            "Unable to fetch project {}. Error: {}",
            project_id,
            response.status()
        ))
    }
}

pub async fn fetch_labels(ctx: &AuthContext, project_id: &str) -> Result<Vec<Label>> {
    let url = format!("{}/projects/{}/labels", ctx.api_url.as_str(), project_id);
    let response = Client::new()
        .get(url)
        .header(reqwest::header::USER_AGENT, USER_AGENT)
        .header(reqwest::header::CONTENT_TYPE, JSON_CONTENT_TYPE)
        .bearer_auth(ctx.token.as_str())
        .send()
        .await?;

    if response.status().is_success() {
        let labels: Vec<Label> = response.json().await?;
        Ok(labels)
    } else {
        Err(anyhow!(
            "Unable to fetch project labels {}. Error: {}",
            project_id,
            response.status()
        ))
    }
}

pub async fn fetch_statuses(ctx: &AuthContext, project_id: &str) -> Result<Vec<IssueStatus>> {
    let url = format!(
        "{}/projects/{}/issueStatuses",
        ctx.api_url.as_str(),
        project_id
    );
    let response = Client::new()
        .get(url)
        .header(reqwest::header::USER_AGENT, USER_AGENT)
        .header(reqwest::header::CONTENT_TYPE, JSON_CONTENT_TYPE)
        .bearer_auth(ctx.token.as_str())
        .send()
        .await?;

    if response.status().is_success() {
        let statuses: Vec<IssueStatus> = response.json().await?;
        Ok(statuses)
    } else {
        Err(anyhow!(
            "Unable to fetch project issue statuses {}. Error: {}",
            project_id,
            response.status()
        ))
    }
}
