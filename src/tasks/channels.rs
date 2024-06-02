use anyhow::anyhow;
use reqwest::Client;

use crate::{
    error::Result,
    models::{auth::AuthContext, channel::ChannelKey},
};

use super::{JSON_CONTENT_TYPE, USER_AGENT};

pub async fn fetch_project_channel(ctx: &AuthContext, project_id: &str) -> Result<ChannelKey> {
    let url = format!(
        "{}/projects/{}/channels/key",
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
        let key: ChannelKey = response.json().await?;
        Ok(key)
    } else {
        Err(anyhow!(
            "Unable to fetch project channel key. Error: {}",
            response.status()
        ))
    }
}
