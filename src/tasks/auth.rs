use anyhow::anyhow;
use reqwest::Client;

use crate::{
    error::Result,
    models::{
        auth::{AuthContext, AuthPayload, AuthSuccess},
        authz::Authz,
    },
};

use super::{JSON_CONTENT_TYPE, USER_AGENT};

pub async fn authenticate(api_url: &str, payload: AuthPayload) -> Result<AuthContext> {
    let url = format!("{}/auth/token/email", api_url);
    let post_body = serde_json::to_string(&payload)?;
    let response = Client::new()
        .post(url)
        .body(post_body)
        .header(reqwest::header::USER_AGENT, USER_AGENT)
        .header(reqwest::header::CONTENT_TYPE, JSON_CONTENT_TYPE)
        .send()
        .await?;

    if !response.status().is_success() {
        return Err(anyhow!(
            "Unable to authenticate. Error: {}",
            response.status()
        ));
    }

    let auth_data: AuthSuccess = response.json().await?;
    let Some(token) = auth_data.token else {
        return Err(anyhow!("Unable to authenticate. No token received."));
    };
    Ok(AuthContext {
        api_url: api_url.to_string(),
        token,
    })
}

pub async fn fetch_project_authz(ctx: &AuthContext, project_id: &str) -> Result<Authz> {
    let url = format!(
        "{}/user/authContext/projects/{}",
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
        let authz: Authz = response.json().await?;
        Ok(authz)
    } else {
        Err(anyhow!(
            "Unable to fetch project permissions {}. Error: {}",
            project_id,
            response.status()
        ))
    }
}
