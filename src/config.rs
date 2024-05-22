use clap::{Parser, Subcommand};
use serde::Deserialize;
use std::path::Path;
use std::{fs, path::PathBuf};

#[derive(Clone, Deserialize, Debug)]
pub struct Config {
    pub token: String,
    pub base_url: String,
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

        if config.issue_count == 0 || config.issue_count > 100 {
            return Err("Issue count must be between 1 to 100");
        }

        // Validate issue type if present
        if let Some(issue_type) = &config.issue_type {
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

            if !issue_types.contains(&issue_type) {
                return Err("Issue type is invalid.");
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
    /// Create issues into project specified in config file
    Create,

    /// Crawl all issues of the specified project
    CrawlIssues,

    /// Craw all issues from all visible projects
    CrawlAllIssues,
}
