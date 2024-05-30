use anyhow::anyhow;
use bigdecimal::BigDecimal;
use std::time::Instant;

use fake::faker::company::en::CatchPhase;
use fake::Fake;
use rand::Rng;

use crate::config::{Config, SingleTargetConfig};
use crate::crawler::{
    create_issue, fetch_epics, fetch_initiatives, fetch_issue, fetch_issues, fetch_labels,
    fetch_me, fetch_members, fetch_project, fetch_projects, fetch_statuses,
};
use crate::error::Result;
use crate::models::issue::{CreateIssueBody, Issue};
use crate::models::issue_status::IssueStatus;
use crate::models::pagination::PaginationResult;
use crate::models::project::{Project, ProjectSlim};

pub async fn run(config: Config) -> Result<()> {
    let timer = Instant::now();
    let current_user = fetch_me(&config).await?;
    println!("Logged in as: {}", current_user.username);

    let Some(single_target_value) = &config.single_target else {
        return Err(anyhow!("Single target config must be present."));
    };

    let single_target = single_target_value.clone();
    let project_id = single_target.project_id.as_str();
    let project = fetch_project(&config, project_id).await?;
    println!("{}: {}", project.key, project.name);

    // Collect statuses and labels
    let labels = fetch_labels(&config, project_id).await?;
    let mut statuses = fetch_statuses(&config, project_id).await?;

    // Remove last status, should not create issues as done
    if statuses.len() > 0 {
        statuses.pop();
    }

    let initiatives = fetch_initiatives(&config, project_id).await?;
    let epics = fetch_epics(&config, project_id).await?;
    let members = fetch_members(&config, project_id).await?;
    let hours = vec![
        1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20,
    ];
    let points = vec![1, 2, 3, 5, 8, 13, 21];

    let project_preferences = project.preferences.unwrap();

    // Default issue type can be configured
    let issue_type = match single_target.issue_type {
        Some(value) => value.as_str(),
        None => project_preferences.issue_type.as_str(),
    };

    let create_timer = Instant::now();

    let mut handles = vec![];

    for _ in 0..single_target.issue_count {
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

        let title: String = CatchPhase().fake();
        let description = format!(
            "{}, {}, {}, {}",
            CatchPhase().fake::<String>(),
            CatchPhase().fake::<String>(),
            CatchPhase().fake::<String>(),
            CatchPhase().fake::<String>()
        );

        let mut payload = CreateIssueBody {
            r#type: issue_type.to_string(),
            initiative_id: None,
            epic_id: None,
            parent_id: None,
            assignee_id: None,
            title,
            description: Some(description),
            estimate_type: Some(project_preferences.estimate_type.clone()),
            estimate: Some(10),
            status: None,
            labels: default_labels,
        };

        if project_preferences.estimate_type == String::from("points") {
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

        let config_copy = config.clone();
        let handle = tokio::spawn(async move {
            create_issue(&config_copy, project_id, &payload)
                .await
                .unwrap()
        });

        handles.push(handle);
    }

    // Gather stats
    let total_reqs: u32 = handles.len().try_into().unwrap();
    let mut failed: u32 = 0;
    let mut min_duration: u128 = 0;
    let mut max_duration: u128 = 0;
    let mut sum: u128 = 0;

    for handle in handles {
        let res = handle.await.unwrap();
        if let None = res.data {
            failed += 1;
        }

        sum += res.duration;

        if min_duration == 0 {
            min_duration = res.duration;
        } else if res.duration < min_duration {
            min_duration = res.duration;
        }

        if res.duration > max_duration {
            max_duration = res.duration;
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

fn get_item_chance(chance: u32) -> bool {
    if chance > 100 {
        panic!("Chance must be between 0 to 100")
    }

    let value = rand::thread_rng().gen_range(0..=100);
    value <= chance
}

fn get_random_item<T>(items: &Vec<T>, chance: u32) -> Option<&T> {
    let length = items.len();
    let return_item = get_item_chance(chance);

    if length > 0 && return_item {
        let max_length = length - 1;
        let key = rand::thread_rng().gen_range(0..=max_length);
        return items.get(key);
    }
    None
}

pub async fn crawl_project_issues(
    config: Config,
    single_target: &SingleTargetConfig,
) -> Result<()> {
    let timer = Instant::now();
    let current_user = fetch_me(&config).await?;
    println!("Logged in as: {}", current_user.username);

    let project_id = single_target.project_id.as_str();
    let project = fetch_project(&config, project_id).await?;
    println!("{}: {}", project.key, project.name);

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
        let listing = fetch_issues(&config, project_id, page, 50).await?;

        has_more = false;
        if listing.data.len() > 0 && listing.meta.total_records > 0 {
            // Queue current batch
            let mut handles = vec![];
            for issue in listing.data {
                let config_copy = config.clone();
                let issue_id = issue.id.clone();
                let handle = tokio::spawn(async move {
                    fetch_issue(&config_copy, project_id, issue_id.as_str())
                        .await
                        .unwrap()
                });

                handles.push(handle);
            }

            let req_count: u32 = handles.len().try_into().unwrap();
            total_reqs += req_count;

            // Process batch
            for handle in handles {
                let res = handle.await.unwrap();
                if let None = res.data {
                    failed += 1;
                }

                sum += res.duration;

                if min_duration == 0 {
                    min_duration = res.duration;
                } else if res.duration < min_duration {
                    min_duration = res.duration;
                }

                if res.duration > max_duration {
                    max_duration = res.duration;
                }
            }

            //
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

pub async fn crawl_all_projects_issues(config: Config) -> Result<()> {
    let timer = Instant::now();
    let current_user = fetch_me(&config).await?;
    println!("Logged in as: {}", current_user.username);

    let projects = collect_projects(&config).await?;
    println!("Visible projects: {}", projects.len());

    let crawl_timer = Instant::now();

    // Gather stats
    let mut total_reqs: u32 = 0;
    let mut failed: u32 = 0;
    let mut min_duration: u128 = 0;
    let mut max_duration: u128 = 0;
    let mut sum: u128 = 0;

    for project in projects {
        println!(
            "Crawling issues for project {}:{}",
            project.key, project.name
        );
        let project_id = project.id;

        let mut has_more = true;
        let mut page = 1;

        while has_more {
            // Fetch listing
            let listing = fetch_issues(&config, project_id.as_str(), page, 50).await?;

            has_more = false;
            if listing.data.len() > 0 && listing.meta.total_records > 0 {
                // Queue current batch
                let mut handles = vec![];
                for issue in listing.data {
                    let config_copy = config.clone();
                    let project_id_copy = project_id.clone();
                    let issue_id = issue.id.clone();
                    let handle = tokio::spawn(async move {
                        fetch_issue(&config_copy, project_id_copy.as_str(), issue_id.as_str())
                            .await
                            .unwrap()
                    });

                    handles.push(handle);
                }

                let req_count: u32 = handles.len().try_into().unwrap();
                total_reqs += req_count;

                // Process batch
                for handle in handles {
                    let res = handle.await.unwrap();
                    if let None = res.data {
                        failed += 1;
                    }

                    sum += res.duration;

                    if min_duration == 0 {
                        min_duration = res.duration;
                    } else if res.duration < min_duration {
                        min_duration = res.duration;
                    }

                    if res.duration > max_duration {
                        max_duration = res.duration;
                    }
                }

                // See if there are still more items
                if listing.meta.total_pages > page {
                    page += 1;
                    has_more = true;
                }
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

async fn collect_projects(config: &Config) -> Result<Vec<ProjectSlim>> {
    let mut ids: Vec<ProjectSlim> = Vec::new();

    let mut has_more = true;
    let mut page = 1;

    while has_more {
        let listing: PaginationResult<Project> = fetch_projects(config, page, 50).await?;
        has_more = false;
        if listing.data.len() > 0 && listing.meta.total_records > 0 {
            for project in listing.data {
                ids.push(ProjectSlim {
                    id: project.id,
                    key: project.key,
                    name: project.name,
                });
            }
            // See if there are still more items
            if listing.meta.total_pages > page {
                page += 1;
                has_more = true;
            }
        }
    }

    Ok(ids)
}
