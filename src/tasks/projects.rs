use std::time::{Duration, Instant};

use anyhow::anyhow;
use fake::Fake;
use fake::faker::company::en::CatchPhrase;
use reqwest::Client;
use tokio::time::sleep;
use tracing::{error, info};

use crate::{
    error::Result,
    models::{
        auth::AuthContext,
        issue_status::IssueStatus,
        label::Label,
        pagination::PaginationResult,
        project::{CreateProjectBody, Project, generate_project_key},
        response_data::ResponseData,
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

pub async fn fetch_project_with_retries(
    ctx: &AuthContext,
    project_id: &str,
    retries: u32,
) -> Result<Project> {
    let mut attempts: u32 = 0;

    loop {
        match fetch_project(ctx, project_id).await {
            Ok(project) => return Ok(project),
            Err(e) => {
                error!("Error: {}", e);

                attempts += 1;
                if attempts > retries {
                    break;
                }

                sleep(Duration::from_secs(3)).await;
            }
        }
    }

    Err(anyhow!("Unable to fetch project {}", project_id,))
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

pub async fn create_project(ctx: &AuthContext) -> Result<Project> {
    // Create project
    let name: String = CatchPhrase().fake();
    let key = generate_project_key(name.as_str());
    let description = format!(
        "{}, {}, {}, {}",
        CatchPhrase().fake::<String>(),
        CatchPhrase().fake::<String>(),
        CatchPhrase().fake::<String>(),
        CatchPhrase().fake::<String>()
    );
    let payload = CreateProjectBody {
        key,
        name,
        description: Some(description),
        issue_type: Some("user_story".to_string()),
        workflow_type: Some("scrum".to_string()),
        estimate_type: Some("hours".to_string()),
        backlog_on_board: Some(true),
    };

    let mut res: ResponseData<Project> = ResponseData {
        duration: 0,
        data: None,
    };

    let d = Instant::now();
    let create_res = do_create_project(ctx, &payload).await;
    res.duration = d.elapsed().as_millis();
    match create_res {
        Ok(project_res) => {
            info!(
                "{}: {} --> {} ms",
                project_res.key, project_res.name, res.duration
            );
            Ok(project_res)
        }
        Err(e) => Err(anyhow!("Error: {}", e)),
    }
}

async fn do_create_project(ctx: &AuthContext, payload: &CreateProjectBody) -> Result<Project> {
    let url = format!("{}/projects", ctx.api_url.as_str());
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
        let project: Project = response.json().await?;
        Ok(project)
    } else {
        let message = format!("Unable to create issue. Error: {}", response.status());
        error!("{}", message);
        Err(anyhow!(message))
    }
}
