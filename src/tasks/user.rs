use anyhow::anyhow;
use reqwest::Client;

use crate::{
    error::Result,
    models::{auth::AuthContext, user::User, user_preference::UserPreference},
};

use super::{JSON_CONTENT_TYPE, USER_AGENT};

pub async fn fetch_current_user(ctx: &AuthContext) -> Result<User> {
    let url = format!("{}/user", ctx.api_url);
    let response = Client::new()
        .get(url)
        .header(reqwest::header::USER_AGENT, USER_AGENT)
        .header(reqwest::header::CONTENT_TYPE, JSON_CONTENT_TYPE)
        .bearer_auth(ctx.token.as_str())
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

pub async fn fetch_user_preferences(ctx: &AuthContext) -> Result<Vec<UserPreference>> {
    let url = format!("{}/user/preferences", ctx.api_url);
    let response = Client::new()
        .get(url)
        .header(reqwest::header::USER_AGENT, USER_AGENT)
        .header(reqwest::header::CONTENT_TYPE, JSON_CONTENT_TYPE)
        .bearer_auth(ctx.token.as_str())
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
