use anyhow::anyhow;
use reqwest::Client;

use crate::{
    error::Result,
    models::auth::{AuthContext, AuthPayload, AuthSuccess},
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
