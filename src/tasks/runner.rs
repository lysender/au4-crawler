use anyhow::anyhow;
use bigdecimal::BigDecimal;
use fake::Fake;
use fake::faker::company::en::CatchPhrase;
use rand::Rng;
use std::time::Instant;
use tokio::task::JoinSet;
use tracing::{error, info};

use crate::{
    config::{Config, GlobalConfig, SingleTargetConfig},
    error::Result,
    models::{
        auth::{AuthContext, AuthPayload},
        issue::{CreateIssueBody, Issue},
        issue_status::IssueStatus,
    },
    tasks::{
        auth::authenticate,
        iam::fetch_project_members,
        issues::{create_issue, fetch_epics, fetch_initiatives, fetch_issue, fetch_issues},
        projects::{fetch_labels, fetch_project, fetch_statuses},
    },
    token::create_captcha_token,
};

pub async fn run_create_issues(config: Config) -> Result<()> {
    let timer = Instant::now();
    // Authenticate
    let api_url = config.global.api_url.as_str();
    let jwt_secret = config.global.jwt_secret.as_str();
    let Some(target) = config.single_target else {
        return Err(anyhow!("Single target config must be present."));
    };
    let captcha_token = create_captcha_token(jwt_secret)?;
    let payload = AuthPayload {
        username: target.username,
        password: target.password,
        captcha_token,
    };
    let context = authenticate(api_url, payload).await?;
    info!("Logged in as {}", context.user.username);

    let project_id = target.project_id.as_str();
    let project = fetch_project(&context, project_id).await?;
    info!("{}: {}", project.key, project.name);

    let labels = fetch_labels(&context, project_id).await?;

    let mut statuses = fetch_statuses(&context, project_id).await?;
    // Remove last status, should not create issues as done
    if statuses.len() > 0 {
        statuses.pop();
    }

    let initiatives = fetch_initiatives(&context, project_id).await?;
    let epics = fetch_epics(&context, project_id).await?;
    let members = fetch_project_members(&context, project_id).await?;
    let hours = vec![
        1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20,
    ];
    let points = vec![1, 2, 3, 5, 8, 13, 21];

    let Some(pref) = project.preferences else {
        return Err(anyhow!("Project preferences must be present."));
    };

    let issue_type = match target.issue_type.as_ref() {
        Some(value) => value.as_str(),
        None => pref.issue_type.as_str(),
    };

    let create_timer = Instant::now();

    let mut set = JoinSet::new();

    for _ in 0..target.issue_count {
        let member = get_random_item(&members, 30);
        let label = get_random_item(&labels, 30);

        let mut initiative: Option<&Issue> = None;
        let mut epic: Option<&Issue> = None;
        let mut status: Option<&IssueStatus> = None;

        // Initiatives and epics do not have these properties
        match issue_type {
            "initiative" => {
                // Do nothing...
            }
            "epic" => {
                initiative = get_random_item(&initiatives, 20);
            }
            _ => {
                epic = get_random_item(&epics, 20);
                status = get_random_item(&statuses, 100);
            }
        };

        let default_labels: Vec<String> = vec![];

        let title: String = CatchPhrase().fake();
        let description = format!(
            "{}, {}, {}, {}",
            CatchPhrase().fake::<String>(),
            CatchPhrase().fake::<String>(),
            CatchPhrase().fake::<String>(),
            CatchPhrase().fake::<String>()
        );

        let mut payload = CreateIssueBody {
            r#type: issue_type.to_string(),
            initiative_id: None,
            epic_id: None,
            parent_id: None,
            assignee_id: None,
            title,
            description: Some(description),
            estimate_type: Some(pref.estimate_type.clone()),
            estimate: Some(10),
            status: None,
            labels: default_labels,
        };

        if pref.estimate_type == String::from("points") {
            let estimate = get_random_item(&points, 100);
            payload.estimate = Some(*estimate.unwrap());
        } else {
            let estimate = get_random_item(&hours, 100);
            payload.estimate = Some(*estimate.unwrap());
        }

        if let Some(initiative_value) = initiative {
            payload.initiative_id = Some(String::from(initiative_value.id.as_str()));
        }

        if let Some(epic_value) = epic {
            payload.epic_id = Some(String::from(epic_value.id.as_str()));
        }
        if let Some(member_value) = member {
            if let Some(user_value) = &member_value.user {
                payload.assignee_id = Some(String::from(user_value.id.as_str()));
            }
        }
        if let Some(status_value) = status {
            payload.status = Some(String::from(status_value.id.as_str()));
        }
        if let Some(label_value) = label {
            payload.labels = vec![String::from(label_value.id.as_str())];
        }

        // Why do I have to clone everything?
        let context_copy = context.clone();
        let payload_copy = payload.clone();
        let project_id_copy = project_id.to_string();

        set.spawn(async move {
            create_issue(&context_copy, project_id_copy.as_str(), &payload_copy).await
        });
    }

    // Gather stats
    let total_reqs: u32 = set.len() as u32;
    let mut failed: u32 = 0;
    let mut min_duration: u128 = 0;
    let mut max_duration: u128 = 0;
    let mut sum: u128 = 0;

    while let Some(join_res) = set.join_next().await {
        match join_res {
            Ok(res) => match res {
                Ok(issue_res) => {
                    if let None = issue_res.data {
                        failed += 1;
                    }

                    sum += issue_res.duration;

                    if min_duration == 0 {
                        min_duration = issue_res.duration;
                    } else if issue_res.duration < min_duration {
                        min_duration = issue_res.duration;
                    }

                    if issue_res.duration > max_duration {
                        max_duration = issue_res.duration;
                    }
                }
                Err(create_err) => {
                    error!("Error: {:?}", create_err);
                }
            },
            Err(join_err) => {
                error!("Error: {:?}", join_err);
            }
        }
    }

    let succeed = total_reqs - failed;
    let big_success_ratio =
        (BigDecimal::from(succeed) / BigDecimal::from(total_reqs)) * BigDecimal::from(100);
    let success_ratio = big_success_ratio.round(2);
    let big_sum = BigDecimal::from(sum);
    let big_total_reqs = BigDecimal::from(total_reqs);
    let big_avg = big_sum / big_total_reqs.clone();
    let avg = big_avg.round(2);

    let total_time = timer.elapsed().as_millis();
    let total_create_time = create_timer.elapsed().as_millis();
    let big_create_total_time = BigDecimal::from(total_create_time);
    let big_rps: BigDecimal = big_total_reqs / (big_create_total_time / 1000.0);
    let rps = big_rps.round(2);

    // Print stats
    println!("");
    println!("Total requests: {}", total_reqs);
    println!("Succeed: {}", succeed);
    println!("Failed: {}", failed);
    println!("Success rate: {}%", success_ratio);
    println!("Min: {} ms", min_duration);
    println!("Avg: {} ms", avg);
    println!("Max: {} ms", max_duration);
    println!("Requests per second: {}", rps);
    println!("Run duration: {} ms", total_time);

    Ok(())
}

pub async fn run_crawl_issues(config: Config) -> Result<()> {
    let timer = Instant::now();
    // Authenticate
    let api_url = config.global.api_url.as_str();
    let jwt_secret = config.global.jwt_secret.as_str();
    let Some(target) = config.single_target else {
        return Err(anyhow!("Single target config must be present."));
    };
    let captcha_token = create_captcha_token(jwt_secret)?;
    let payload = AuthPayload {
        username: target.username,
        password: target.password,
        captcha_token,
    };
    let context = authenticate(api_url, payload).await?;
    info!("Logged in as {}", context.user.username);

    let project_id = target.project_id.as_str();
    let project = fetch_project(&context, project_id).await?;
    info!("{}: {}", project.key, project.name);

    let crawl_timer = Instant::now();

    // Gather stats
    let mut total_reqs: u32 = 0;
    let mut failed: u32 = 0;
    let mut min_duration: u128 = 0;
    let mut max_duration: u128 = 0;
    let mut sum: u128 = 0;

    let mut has_more = true;
    let mut page = 1;

    while has_more {
        // Fetch listing
        let listing = fetch_issues(&context, project_id, page, 50).await?;

        has_more = false;
        if listing.data.len() > 0 && listing.meta.total_records > 0 {
            // Queue current batch
            let mut set = JoinSet::new();
            for issue in listing.data {
                let context_copy = context.clone();
                let issue_id = issue.id.clone();
                let project_id_copy = target.project_id.clone();

                set.spawn(async move {
                    fetch_issue(&context_copy, project_id_copy.as_str(), issue_id.as_str()).await
                });
            }

            let req_count: u32 = set.len() as u32;
            total_reqs += req_count;

            // Process batch
            while let Some(join_res) = set.join_next().await {
                match join_res {
                    Ok(res) => match res {
                        Ok(issue_res) => {
                            if let None = issue_res.data {
                                failed += 1;
                            }

                            sum += issue_res.duration;

                            if min_duration == 0 {
                                min_duration = issue_res.duration;
                            } else if issue_res.duration < min_duration {
                                min_duration = issue_res.duration;
                            }

                            if issue_res.duration > max_duration {
                                max_duration = issue_res.duration;
                            }
                        }
                        Err(issue_err) => {
                            error!("Error: {:?}", issue_err);
                        }
                    },
                    Err(err) => {
                        error!("Error: {:?}", err);
                    }
                }
            }

            // See if there are still more items
            if listing.meta.total_pages > page {
                page += 1;
                has_more = true;
            }
        }
    }

    let succeed = total_reqs - failed;
    let big_success_ratio =
        (BigDecimal::from(succeed) / BigDecimal::from(total_reqs)) * BigDecimal::from(100);
    let success_ratio = big_success_ratio.round(2);
    let big_sum = BigDecimal::from(sum);
    let big_total_reqs = BigDecimal::from(total_reqs);
    let big_avg = big_sum / big_total_reqs.clone();
    let avg = big_avg.round(2);

    let total_time = timer.elapsed().as_millis();
    let total_crawl_time = crawl_timer.elapsed().as_millis();
    let big_crawl_total_time = BigDecimal::from(total_crawl_time);
    let big_rps: BigDecimal = big_total_reqs / (big_crawl_total_time / 1000.0);
    let rps = big_rps.round(2);

    // Print stats
    println!("");
    println!("Total requests: {}", total_reqs);
    println!("Succeed: {}", succeed);
    println!("Failed: {}", failed);
    println!("Success rate: {}%", success_ratio);
    println!("Min: {} ms", min_duration);
    println!("Avg: {} ms", avg);
    println!("Max: {} ms", max_duration);
    println!("Requests per second: {}", rps);
    println!("Run duration: {} ms", total_time);

    Ok(())
}

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

fn get_item_chance(chance: u32) -> bool {
    if chance > 100 {
        panic!("Chance must be between 0 to 100")
    }

    let value = rand::rng().random_range(0..=100);
    value <= chance
}

fn get_random_item<T>(items: &Vec<T>, chance: u32) -> Option<&T> {
    let length = items.len();
    let return_item = get_item_chance(chance);

    if length > 0 && return_item {
        let max_length = length - 1;
        let key = rand::rng().random_range(0..=max_length);
        return items.get(key);
    }
    None
}
