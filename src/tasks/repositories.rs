use anyhow::anyhow;
use reqwest::Client;

use crate::{
    error::Result,
    models::{auth::AuthContext, repository::Repository},
};

use super::{JSON_CONTENT_TYPE, USER_AGENT};

pub async fn fetch_project_repositories(
    ctx: &AuthContext,
    project_id: &str,
) -> Result<Vec<Repository>> {
    let url = format!(
        "{}/projects/{}/repositories",
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
        let list: Vec<Repository> = response.json().await?;
        Ok(list)
    } else {
        Err(anyhow!(
            "Unable to fetch project repositories. Error: {}",
            response.status()
        ))
    }
}
