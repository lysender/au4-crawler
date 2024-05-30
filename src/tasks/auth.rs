use anyhow::anyhow;
use reqwest::Client;

use crate::{
    error::Result,
    models::auth::{AuthPayload, AuthSuccess},
};

use super::{JSON_CONTENT_TYPE, USER_AGENT};

pub async fn authenticate(
    base_url: &str,
    token: &str,
    username: &str,
    password: &str,
) -> Result<AuthSuccess> {
    let url = format!("{}/auth/token/email", base_url);
    let payload = AuthPayload {
        username: username.to_string(),
        password: password.to_string(),
        captcha_token: "".to_string(),
    };
    let post_body = serde_json::to_string(&payload)?;
    let response = Client::new()
        .post(url)
        .body(post_body)
        .header(reqwest::header::USER_AGENT, USER_AGENT)
        .header(reqwest::header::CONTENT_TYPE, JSON_CONTENT_TYPE)
        .bearer_auth(token)
        .send()
        .await?;

    if response.status().is_success() {
        let auth_data: AuthSuccess = response.json().await?;
        Ok(auth_data)
    } else {
        Err(anyhow!(
            "Unable to authenticate. Error: {}",
            response.status()
        ))
    }
}
