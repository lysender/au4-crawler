use anyhow::anyhow;
use reqwest::Client;

use crate::{
    error::Result,
    models::{
        actor::Actor, auth::AuthContext, organisation::Organisation, project_member::ProjectMember,
    },
};

use super::{JSON_CONTENT_TYPE, USER_AGENT};

pub async fn fetch_iam(ctx: &AuthContext) -> Result<Actor> {
    let url = format!("{}/iam", ctx.api_url);
    let response = Client::new()
        .get(url)
        .header(reqwest::header::USER_AGENT, USER_AGENT)
        .header(reqwest::header::CONTENT_TYPE, JSON_CONTENT_TYPE)
        .bearer_auth(ctx.token.as_str())
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

pub async fn fetch_my_organisation(ctx: &AuthContext) -> Result<Organisation> {
    let url = format!("{}/iam/organisation", ctx.api_url);
    let response = Client::new()
        .get(url)
        .header(reqwest::header::USER_AGENT, USER_AGENT)
        .header(reqwest::header::CONTENT_TYPE, JSON_CONTENT_TYPE)
        .bearer_auth(ctx.token.as_str())
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

pub async fn fetch_project_members(
    ctx: &AuthContext,
    project_id: &str,
) -> Result<Vec<ProjectMember>> {
    let url = format!(
        "{}/iam/projects/{}/members/?status=active",
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
        let members: Vec<ProjectMember> = response.json().await?;
        Ok(members)
    } else {
        Err(anyhow!(
            "Unable to fetch project members. Error: {}",
            response.status()
        ))
    }
}
