use anyhow::anyhow;
use reqwest::Client;
use std::time::Instant;

use crate::config::Config;
use crate::error::Result;
use crate::models::activity_log::ActivityLog;
use crate::models::actor::Actor;
use crate::models::authz::Authz;
use crate::models::channel::ChannelKey;
use crate::models::comment::Comment;
use crate::models::issue::{CreateIssueBody, Issue};
use crate::models::issue_status::IssueStatus;
use crate::models::label::Label;
use crate::models::organisation::Organisation;
use crate::models::pagination::PaginationResult;
use crate::models::project::Project;
use crate::models::project_member::ProjectMember;
use crate::models::repository::Repository;
use crate::models::user::User;
use crate::models::user_preference::UserPreference;

#[derive(Debug)]
pub struct ResponseData<T> {
    pub duration: u128,
    pub data: Option<T>,
}

const USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/118.0.0.0 Safari/537.36";
const JSON_CONTENT_TYPE: &str = "application/json";

pub async fn fetch_iam(config: &Config) -> Result<Actor> {
    let url = format!("{}/iam", config.base_url);
    let response = Client::new()
        .get(url)
        .header(reqwest::header::USER_AGENT, USER_AGENT)
        .header(reqwest::header::CONTENT_TYPE, JSON_CONTENT_TYPE)
        .bearer_auth(config.token.as_str())
        .send()
        .await?;

    if response.status().is_success() {
        let actor: Actor = response.json().await?;
        Ok(actor)
    } else {
        Err(anyhow!(
            "Unable to fetch current actor. Error: {}",
            response.status()
        ))
    }
}

pub async fn fetch_my_organisation(config: &Config) -> Result<Organisation> {
    let url = format!("{}/iam/organisation", config.base_url);
    let response = Client::new()
        .get(url)
        .header(reqwest::header::USER_AGENT, USER_AGENT)
        .header(reqwest::header::CONTENT_TYPE, JSON_CONTENT_TYPE)
        .bearer_auth(config.token.as_str())
        .send()
        .await?;

    if response.status().is_success() {
        let org: Organisation = response.json().await?;
        Ok(org)
    } else {
        Err(anyhow!(
            "Unable to fetch current organisation. Error: {}",
            response.status()
        ))
    }
}

pub async fn fetch_me(config: &Config) -> Result<User> {
    let url = format!("{}/user", config.base_url);
    let response = Client::new()
        .get(url)
        .header(reqwest::header::USER_AGENT, USER_AGENT)
        .header(reqwest::header::CONTENT_TYPE, JSON_CONTENT_TYPE)
        .bearer_auth(config.token.as_str())
        .send()
        .await?;

    if response.status().is_success() {
        let user: User = response.json().await?;
        Ok(user)
    } else {
        Err(anyhow!(
            "Unable to fetch current user. Error: {}",
            response.status()
        ))
    }
}

pub async fn fetch_user_preferences(config: &Config) -> Result<Vec<UserPreference>> {
    let url = format!("{}/user/preferences", config.base_url);
    let response = Client::new()
        .get(url)
        .header(reqwest::header::USER_AGENT, USER_AGENT)
        .header(reqwest::header::CONTENT_TYPE, JSON_CONTENT_TYPE)
        .bearer_auth(config.token.as_str())
        .send()
        .await?;

    if response.status().is_success() {
        let prefs: Vec<UserPreference> = response.json().await?;
        Ok(prefs)
    } else {
        Err(anyhow!(
            "Unable to fetch current user preferences. Error: {}",
            response.status()
        ))
    }
}
pub async fn fetch_projects(
    config: &Config,
    page: u32,
    per_page: u32,
) -> Result<PaginationResult<Project>> {
    let url = format!("{}/projects", config.base_url.as_str());
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
        .bearer_auth(config.token.as_str())
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

pub async fn fetch_project(config: &Config, project_id: &str) -> Result<Project> {
    let url = format!("{}/projects/{}", config.base_url.as_str(), project_id);
    let query_params = vec![("include", "organisation".to_string())];
    let response = Client::new()
        .get(url)
        .query(&query_params)
        .header(reqwest::header::USER_AGENT, USER_AGENT)
        .header(reqwest::header::CONTENT_TYPE, JSON_CONTENT_TYPE)
        .bearer_auth(config.token.as_str())
        .send()
        .await?;

    if response.status().is_success() {
        let project: Project = response.json().await?;
        Ok(project)
    } else {
        Err(anyhow!(
            "Unable to fetch project {}. Error: {}",
            config.project_id.as_str(),
            response.status()
        ))
    }
}

pub async fn fetch_project_authz(config: &Config, project_id: &str) -> Result<Authz> {
    let url = format!(
        "{}/user/authContext/projects/{}",
        config.base_url.as_str(),
        project_id
    );
    let response = Client::new()
        .get(url)
        .header(reqwest::header::USER_AGENT, USER_AGENT)
        .header(reqwest::header::CONTENT_TYPE, JSON_CONTENT_TYPE)
        .bearer_auth(config.token.as_str())
        .send()
        .await?;

    if response.status().is_success() {
        let authz: Authz = response.json().await?;
        Ok(authz)
    } else {
        Err(anyhow!(
            "Unable to fetch project permissions {}. Error: {}",
            config.project_id.as_str(),
            response.status()
        ))
    }
}

pub async fn fetch_labels(config: &Config, project_id: &str) -> Result<Vec<Label>> {
    let url = format!(
        "{}/projects/{}/labels",
        config.base_url.as_str(),
        project_id
    );
    let response = Client::new()
        .get(url)
        .header(reqwest::header::USER_AGENT, USER_AGENT)
        .header(reqwest::header::CONTENT_TYPE, JSON_CONTENT_TYPE)
        .bearer_auth(config.token.as_str())
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

pub async fn fetch_statuses(config: &Config, project_id: &str) -> Result<Vec<IssueStatus>> {
    let url = format!(
        "{}/projects/{}/issueStatuses",
        config.base_url.as_str(),
        project_id
    );
    let response = Client::new()
        .get(url)
        .header(reqwest::header::USER_AGENT, USER_AGENT)
        .header(reqwest::header::CONTENT_TYPE, JSON_CONTENT_TYPE)
        .bearer_auth(config.token.as_str())
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

pub async fn fetch_initiatives(config: &Config, project_id: &str) -> Result<Vec<Issue>> {
    let url = format!(
        "{}/projects/{}/issues",
        config.base_url.as_str(),
        project_id
    );
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
        .bearer_auth(config.token.as_str())
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
pub async fn fetch_epics(config: &Config, project_id: &str) -> Result<Vec<Issue>> {
    let url = format!(
        "{}/projects/{}/issues",
        config.base_url.as_str(),
        project_id
    );
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
        .bearer_auth(config.token.as_str())
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

pub async fn fetch_members(config: &Config, project_id: &str) -> Result<Vec<ProjectMember>> {
    let url = format!(
        "{}/iam/projects/{}/members/?status=active",
        config.base_url.as_str(),
        project_id
    );
    let response = Client::new()
        .get(url)
        .header(reqwest::header::USER_AGENT, USER_AGENT)
        .header(reqwest::header::CONTENT_TYPE, JSON_CONTENT_TYPE)
        .bearer_auth(config.token.as_str())
        .send()
        .await?;

    if response.status().is_success() {
        let members: Vec<ProjectMember> = response.json().await?;
        Ok(members)
    } else {
        Err(anyhow!(
            "Unable to fetch project members. Error: {}",
            response.status()
        ))
    }
}

pub async fn fetch_project_repositories(
    config: &Config,
    project_id: &str,
) -> Result<Vec<Repository>> {
    let url = format!(
        "{}/projects/{}/repositories",
        config.base_url.as_str(),
        project_id
    );
    let response = Client::new()
        .get(url)
        .header(reqwest::header::USER_AGENT, USER_AGENT)
        .header(reqwest::header::CONTENT_TYPE, JSON_CONTENT_TYPE)
        .bearer_auth(config.token.as_str())
        .send()
        .await?;

    if response.status().is_success() {
        let list: Vec<Repository> = response.json().await?;
        Ok(list)
    } else {
        Err(anyhow!(
            "Unable to fetch project repositories. Error: {}",
            response.status()
        ))
    }
}

pub async fn fetch_project_channel(config: &Config, project_id: &str) -> Result<ChannelKey> {
    let url = format!(
        "{}/projects/{}/channels/key",
        config.base_url.as_str(),
        project_id
    );
    let response = Client::new()
        .get(url)
        .header(reqwest::header::USER_AGENT, USER_AGENT)
        .header(reqwest::header::CONTENT_TYPE, JSON_CONTENT_TYPE)
        .bearer_auth(config.token.as_str())
        .send()
        .await?;

    if response.status().is_success() {
        let key: ChannelKey = response.json().await?;
        Ok(key)
    } else {
        Err(anyhow!(
            "Unable to fetch project channel key. Error: {}",
            response.status()
        ))
    }
}

pub async fn create_issue(
    config: &Config,
    project_id: &str,
    payload: &CreateIssueBody,
) -> Result<ResponseData<Issue>> {
    let mut res: ResponseData<Issue> = ResponseData {
        duration: 0,
        data: None,
    };

    let d = Instant::now();
    let create_res = do_create_issue(config, project_id, payload).await;
    res.duration = d.elapsed().as_millis();
    if let Ok(issue_res) = create_res {
        println!(
            "{}: {} --> {} ms",
            issue_res.key, issue_res.title, res.duration
        );
        res.data = Some(issue_res);
    }

    Ok(res)
}

async fn do_create_issue(
    config: &Config,
    project_id: &str,
    payload: &CreateIssueBody,
) -> Result<Issue> {
    let url = format!(
        "{}/projects/{}/issues",
        config.base_url.as_str(),
        project_id
    );
    let post_body = serde_json::to_string(payload)?;

    let response = Client::new()
        .post(url)
        .header(reqwest::header::USER_AGENT, USER_AGENT)
        .header(reqwest::header::CONTENT_TYPE, JSON_CONTENT_TYPE)
        .body(post_body)
        .bearer_auth(config.token.as_str())
        .send()
        .await?;

    if response.status().is_success() {
        let issue: Issue = response.json().await?;
        Ok(issue)
    } else {
        let message = format!("Unable to create issue. Error: {}", response.status());
        eprintln!("{}", message);
        Err(anyhow!(message))
    }
}

pub async fn fetch_issues(
    config: &Config,
    project_id: &str,
    page: u32,
    per_page: u32,
) -> Result<PaginationResult<Issue>> {
    let url = format!(
        "{}/projects/{}/issues",
        config.base_url.as_str(),
        project_id
    );
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
        .bearer_auth(config.token.as_str())
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
        eprintln!("{}", message);
        Err(anyhow!(message))
    }
}

pub async fn fetch_issue(
    config: &Config,
    project_id: &str,
    issue_id: &str,
) -> Result<ResponseData<Issue>> {
    let mut res: ResponseData<Issue> = ResponseData {
        duration: 0,
        data: None,
    };

    let d = Instant::now();
    let create_res = do_fetch_issue(config, project_id, issue_id).await;
    res.duration = d.elapsed().as_millis();
    if let Ok(issue_res) = create_res {
        println!(
            "{}: {} --> {} ms",
            issue_res.key, issue_res.title, res.duration
        );
        res.data = Some(issue_res);
    }

    Ok(res)
}

async fn do_fetch_issue(config: &Config, project_id: &str, issue_id: &str) -> Result<Issue> {
    let url = format!(
        "{}/projects/{}/issues/{}",
        config.base_url.as_str(),
        project_id,
        issue_id
    );
    let query_params = vec![(
        "include",
        "isCreator,isAssignee,isFollower,initiative,epic,parent,commitment,subtasksCount"
            .to_string(),
    )];

    let response = Client::new()
        .get(url)
        .query(&query_params)
        .header(reqwest::header::USER_AGENT, USER_AGENT)
        .header(reqwest::header::CONTENT_TYPE, JSON_CONTENT_TYPE)
        .bearer_auth(config.token.as_str())
        .send()
        .await?;

    if response.status().is_success() {
        let issue: Issue = response.json().await?;
        Ok(issue)
    } else {
        let message = format!("Unable to fetch issue. Error: {}", response.status());
        eprintln!("{}", message);
        Err(anyhow!(message))
    }
}

pub async fn fetch_issue_comments(
    config: &Config,
    project_id: &str,
    issue_id: &str,
    page: u32,
    per_page: u32,
) -> Result<ResponseData<PaginationResult<Comment>>> {
    let mut res: ResponseData<PaginationResult<Comment>> = ResponseData {
        duration: 0,
        data: None,
    };

    let d = Instant::now();
    let listing_res = do_fetch_issue_comments(config, project_id, issue_id, page, per_page).await;
    res.duration = d.elapsed().as_millis();
    if let Ok(listing) = listing_res {
        res.data = Some(listing);
    }

    Ok(res)
}

pub async fn do_fetch_issue_comments(
    config: &Config,
    project_id: &str,
    issue_id: &str,
    page: u32,
    per_page: u32,
) -> Result<PaginationResult<Comment>> {
    let url = format!(
        "{}/projects/{}/issues/{}/comments",
        config.base_url.as_str(),
        project_id,
        issue_id
    );
    let query_params = vec![
        ("page", page.to_string()),
        ("per_page", per_page.to_string()),
        ("sort", "-createdAt".to_string()),
    ];

    let response = Client::new()
        .get(url)
        .query(&query_params)
        .header(reqwest::header::USER_AGENT, USER_AGENT)
        .header(reqwest::header::CONTENT_TYPE, JSON_CONTENT_TYPE)
        .bearer_auth(config.token.as_str())
        .send()
        .await?;

    if response.status().is_success() {
        let result: PaginationResult<Comment> = response.json().await?;
        Ok(result)
    } else {
        let message = format!(
            "Unable to fetch issue comment listing. Error: {}",
            response.status()
        );
        eprintln!("{}", message);
        Err(anyhow!(message))
    }
}

pub async fn fetch_issue_timeline_items(
    config: &Config,
    project_id: &str,
    issue_id: &str,
    page: u32,
    per_page: u32,
) -> Result<ResponseData<PaginationResult<ActivityLog>>> {
    let mut res: ResponseData<PaginationResult<ActivityLog>> = ResponseData {
        duration: 0,
        data: None,
    };

    let d = Instant::now();
    let listing_res =
        do_fetch_issue_timeline_items(config, project_id, issue_id, page, per_page).await;
    res.duration = d.elapsed().as_millis();
    if let Ok(listing) = listing_res {
        res.data = Some(listing);
    }

    Ok(res)
}

pub async fn do_fetch_issue_timeline_items(
    config: &Config,
    project_id: &str,
    issue_id: &str,
    page: u32,
    per_page: u32,
) -> Result<PaginationResult<ActivityLog>> {
    let url = format!(
        "{}/projects/{}/issues/{}/timelineitems",
        config.base_url.as_str(),
        project_id,
        issue_id
    );
    let query_params = vec![
        ("page", page.to_string()),
        ("per_page", per_page.to_string()),
        ("sort", "-createdAt".to_string()),
        ("include", "meta,commens".to_string()),
    ];

    let response = Client::new()
        .get(url)
        .query(&query_params)
        .header(reqwest::header::USER_AGENT, USER_AGENT)
        .header(reqwest::header::CONTENT_TYPE, JSON_CONTENT_TYPE)
        .bearer_auth(config.token.as_str())
        .send()
        .await?;

    if response.status().is_success() {
        let result: PaginationResult<ActivityLog> = response.json().await?;
        Ok(result)
    } else {
        let message = format!(
            "Unable to fetch issue timelime itemlisting. Error: {}",
            response.status()
        );
        eprintln!("{}", message);
        Err(anyhow!(message))
    }
}

pub async fn fetch_issue_page_resources(
    config: &Config,
    project_id: &str,
    issue_id: &str,
) -> Result<PaginationResult<Comment>> {
    // Fetch all resources in an issue page
    // Fetch iam
    // Fetch organisation
    // Fetch user preferences
    // Fetch all organisations
    // Fetch recent projects
    // Fetch project
    // Fetch project authz
    // Fetch project channel
    // Fetch project epics
    // Fetch project repositories (if has access)
    // Fetch project labels
    // Fetch project statuses
    // Fetch project members (if has access)
    // Fetch project Github integration (if has access)
    // Fetch my project file quota
    // Fetch account notifications
    // Fetch regular notifications
    // Fetch sprints (if scrum project)
    // Fetch issue
    // Fetch issue files
    // Fetch issue subtasks if issue is not a subtask
    // Fetch issue timeline items
    // Fetch issue followers
    // Fetch development updates (if has Github integration)
    // Fetch issue repo branches
    // Fetch issue comments count
    Err(anyhow!("Error"))
}
