use clap::Parser;
use config::Args;
use config::Commands;
use config::Config;
use std::process;

use crate::error::Result;

pub mod config;
pub mod error;
pub mod models;
pub mod run;
pub mod tasks;
pub mod token;

#[tokio::main]
async fn main() {
    // Set the RUST_LOG, if it hasn't been explicitly defined
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "au4_crawler=info")
    }

    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .init();

    let args = Args::parse();
    let config = Config::build(args.config.as_path()).unwrap_or_else(|err| {
        eprintln!("{err}");
        process::exit(1);
    });

    if let Err(e) = run_command(args, config).await {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}

async fn run_command(args: Args, config: Config) -> Result<()> {
    match args.command {
        Commands::CreateIssues => Ok(()),
        Commands::CrawlIssues => Ok(()),
        Commands::UsersReadonly => Ok(()),
        Commands::UsersReadWrite => Ok(()),
    }
}
