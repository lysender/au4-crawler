pub mod auth;
pub mod comments;
pub mod issues;
pub mod notifications;
pub mod organisations;
pub mod projects;
pub mod runner;
pub mod sprints;
pub mod timeline;

pub const USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/118.0.0.0 Safari/537.36";
pub const JSON_CONTENT_TYPE: &str = "application/json";
