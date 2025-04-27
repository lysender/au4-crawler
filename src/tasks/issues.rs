use anyhow::anyhow;
use reqwest::Client;
use std::time::Instant;
use tracing::{error, info};

use crate::{
    error::Result,
    models::{
        auth::AuthContext,
        issue::{CreateIssueBody, Issue},
        pagination::PaginationResult,
        response_data::ResponseData,
    },
};

use super::{JSON_CONTENT_TYPE, USER_AGENT};

pub async fn fetch_initiatives(ctx: &AuthContext, project_id: &str) -> Result<Vec<Issue>> {
    let url = format!("{}/projects/{}/issues", ctx.api_url.as_str(), project_id);
    let query_params = vec![
        ("type", "initiative".to_string()),
        ("state", "active".to_string()),
        ("page", "1".to_string()),
        ("per_page", "50".to_string()),
        ("sort", "-createdAt".to_string()),
        (
            "include",
            "createdBy,assignee,developmentUpdates,isFollower,subtasksCount".to_string(),
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
        let issues: Vec<Issue> = response.json().await?;
        Ok(issues)
    } else {
        Err(anyhow!(
            "Unable to fetch epics. Error: {}",
            response.status()
        ))
    }
}
pub async fn fetch_epics(ctx: &AuthContext, project_id: &str) -> Result<Vec<Issue>> {
    let url = format!("{}/projects/{}/issues", ctx.api_url.as_str(), project_id);
    let query_params = vec![
        ("type", "epic".to_string()),
        ("state", "active".to_string()),
        ("page", "1".to_string()),
        ("per_page", "50".to_string()),
        ("sort", "-createdAt".to_string()),
        (
            "include",
            "createdBy,assignee,developmentUpdates,isFollower,subtasksCount".to_string(),
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
        let issues: Vec<Issue> = response.json().await?;
        Ok(issues)
    } else {
        Err(anyhow!(
            "Unable to fetch epics. Error: {}",
            response.status()
        ))
    }
}

pub async fn create_issue(
    ctx: &AuthContext,
    project_id: &str,
    payload: &CreateIssueBody,
) -> Result<ResponseData<Issue>> {
    let mut res: ResponseData<Issue> = ResponseData {
        duration: 0,
        data: None,
    };

    let d = Instant::now();
    let create_res = do_create_issue(ctx, project_id, payload).await;
    res.duration = d.elapsed().as_millis();
    if let Ok(issue_res) = create_res {
        info!(
            "{}: {} --> {} ms",
            issue_res.key, issue_res.title, res.duration
        );
        res.data = Some(issue_res);
    }

    Ok(res)
}

async fn do_create_issue(
    ctx: &AuthContext,
    project_id: &str,
    payload: &CreateIssueBody,
) -> Result<Issue> {
    let url = format!("{}/projects/{}/issues", ctx.api_url.as_str(), project_id);
    let post_body = serde_json::to_string(payload)?;

    let response = Client::new()
        .post(url)
        .header(reqwest::header::USER_AGENT, USER_AGENT)
        .header(reqwest::header::CONTENT_TYPE, JSON_CONTENT_TYPE)
        .body(post_body)
        .bearer_auth(ctx.token.as_str())
        .send()
        .await?;

    if response.status().is_success() {
        let issue: Issue = response.json().await?;
        Ok(issue)
    } else {
        let message = format!("Unable to create issue. Error: {}", response.status());
        error!("{}", message);
        Err(anyhow!(message))
    }
}

pub async fn fetch_issues(
    ctx: &AuthContext,
    project_id: Option<&str>,
    page: u32,
    per_page: u32,
) -> Result<PaginationResult<Issue>> {
    let url = match project_id {
        Some(pid) => format!("{}/projects/{}/issues", ctx.api_url.as_str(), pid),
        None => format!("{}/issues", ctx.api_url.as_str()),
    };

    let query_params = vec![
        ("state", "active".to_string()),
        ("page", page.to_string()),
        ("per_page", per_page.to_string()),
        ("sort", "-createdAt".to_string()),
        (
            "include",
            "createdBy,assignee,developmentUpdates,isFollower,subtasksCount,meta".to_string(),
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
        let result: PaginationResult<Issue> = response.json().await?;
        Ok(result)
    } else {
        let message = format!(
            "Unable to fetch issue listing. Error: {}",
            response.status()
        );
        error!("{}", message);
        Err(anyhow!(message))
    }
}

pub async fn fetch_issue(
    ctx: &AuthContext,
    project_id: &str,
    issue_id: &str,
) -> Result<ResponseData<Issue>> {
    let mut res: ResponseData<Issue> = ResponseData {
        duration: 0,
        data: None,
    };

    let d = Instant::now();
    let fetch_res = do_fetch_issue(ctx, project_id, issue_id).await;
    res.duration = d.elapsed().as_millis();

    match fetch_res {
        Ok(issue) => {
            info!("{}: {} --> {} ms", issue.key, issue.title, res.duration);
            res.data = Some(issue);
        }
        Err(e) => {
            error!("Error: {}", e);
        }
    };

    Ok(res)
}

async fn do_fetch_issue(ctx: &AuthContext, project_id: &str, issue_id: &str) -> Result<Issue> {
    let url = format!(
        "{}/projects/{}/issues/{}",
        ctx.api_url.as_str(),
        project_id,
        issue_id
    );
    let query_params = vec![(
        "include",
        "isCreator,isAssignee,isFollower,initiative,epic,parent,commitment,subtasksCount"
            .to_string(),
    )];

    let response = Client::new()
        .get(&url)
        .query(&query_params)
        .header(reqwest::header::USER_AGENT, USER_AGENT)
        .header(reqwest::header::CONTENT_TYPE, JSON_CONTENT_TYPE)
        .bearer_auth(ctx.token.as_str())
        .send()
        .await?;

    if response.status().is_success() {
        let issue: Issue = response.json().await?;
        Ok(issue)
    } else {
        let message = format!(
            "Unable to fetch issue. Error: {}, URL={}",
            response.status(),
            url
        );
        error!("{}", message);
        Err(anyhow!(message))
    }
}
