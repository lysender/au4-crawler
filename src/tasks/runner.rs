use crate::{
    config::{GlobalConfig, SingleTargetConfig},
    error::Result,
    models::auth::{AuthContext, AuthPayload},
    token::create_captcha_token,
};

use super::auth::authenticate;

pub async fn run_user_tasks(config: &GlobalConfig, target: &SingleTargetConfig) -> Result<()> {
    let captcha_token = create_captcha_token(&config.jwt_secret)?;
    let auth_payload = AuthPayload {
        username: target.username.clone(),
        password: target.password.clone(),
        captcha_token,
    };
    let ctx = authenticate(&config.api_url, auth_payload).await?;
    run_tasks(&ctx).await?;

    Ok(())
}

async fn run_tasks(ctx: &AuthContext) -> Result<()> {
    // Fetch IAM
    // Fetch projects
    // For each project, do:
    // Fetch permissions
    // Fetch statuses
    // Fetch labels
    // Fetch first 50 epics
    // Fetch members
    // Do actual tasks...
    Ok(())
}
