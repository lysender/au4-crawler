use clap::{Parser, Subcommand};
use serde::Deserialize;
use std::path::Path;
use std::{fs, path::PathBuf};

#[derive(Clone, Deserialize, Debug)]
pub struct Config {
    pub global: GlobalConfig,
    pub multi_target: Option<MultiTargetConfig>,
    pub single_target: Option<SingleTargetConfig>,
}

#[derive(Clone, Deserialize, Debug)]
pub struct GlobalConfig {
    pub api_url: String,
    pub jwt_secret: String,
}

#[derive(Clone, Deserialize, Debug)]
pub struct MultiTargetConfig {
    pub users: Vec<Credential>,
    pub issue_count: u32,
    pub issue_type: Option<String>,
}

#[derive(Clone, Deserialize, Debug)]
pub struct Credential {
    pub username: String,
    pub password: String,
}

#[derive(Clone, Deserialize, Debug)]
pub struct SingleTargetConfig {
    pub username: String,
    pub password: String,
    pub project_id: String,
    pub issue_count: u32,
    pub issue_type: Option<String>,
}

impl Config {
    pub fn build(filename: &Path) -> Result<Config, &'static str> {
        let toml_string = match fs::read_to_string(filename) {
            Ok(str) => str,
            Err(_) => {
                return Err("Unable to read config file.");
            }
        };

        let config: Config = match toml::from_str(toml_string.as_str()) {
            Ok(value) => value,
            Err(err) => {
                println!("{:?}", err);
                return Err("Unable to parse config file.");
            }
        };

        // At least one of single target or multi-target config must be present
        if config.single_target.is_none() && config.multi_target.is_none() {
            return Err("Either single target or multi-target config must be present.");
        }

        let issue_types = vec![
            String::from("initiative"),
            String::from("epic"),
            String::from("user_story"),
            String::from("task"),
            String::from("issue"),
            String::from("feature"),
            String::from("bug"),
            String::from("test_case"),
        ];

        // Validate multi-target config
        if let Some(multi_target) = &config.multi_target {
            if multi_target.users.is_empty() {
                return Err("At least one user must be present in multi-target config.");
            }
            if multi_target.issue_count == 0 || multi_target.issue_count > 100 {
                return Err("Issue count must be between 1 to 100");
            }
            if let Some(issue_type) = &multi_target.issue_type {
                if !issue_types.contains(&issue_type) {
                    return Err("Issue type is invalid.");
                }
            }
        }

        // Validate single-target config
        if let Some(single_target) = &config.single_target {
            if single_target.project_id.is_empty() {
                return Err("Project ID must be present in single-target config.");
            }
            if single_target.issue_count == 0 || single_target.issue_count > 100 {
                return Err("Issue count must be between 1 to 100");
            }

            // Validate issue type if present
            if let Some(issue_type) = &single_target.issue_type {
                if !issue_types.contains(&issue_type) {
                    return Err("Issue type is invalid.");
                }
            }
        }

        Ok(config)
    }
}

/// CLI tool to create issues into a project
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// TOML configuration file
    #[arg(short, long, value_name = "FILE.toml")]
    pub config: PathBuf,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Create issues into a specific project
    CreateIssues,

    /// Crawl all issues of the specified project
    CrawlIssues,

    /// Simulate all users doing random actions as if they are working in their projects
    UsersReadWrite,

    /// Simulate all users doing random actions as if they are working in their projects
    UsersReadonly,
}
