# Project Crawler 

Project crawler to mimic users doing work

For the secret startup project.

## Objectives

- Simulate concurrent users using the application
- It should do the following tasks:
    - Login
    - Lookup all available projects
    - Walk through each project and perform tasks
    - Visit all endpoints in the following pages under the project:
        - Dashboard
        - Roadmap
        - Board
        - Backlog
        - Epics
        - Issues
        - Code repos
        - Team
        - Reports
        - Files
        - Settings
        - Trash
    - Perform the following actions:
        - Create a random number of issues
        - Create a random number of sprints
        - Move a random number of issues in the board one step towards completion
        - If all issues in the board are all done, finish the sprint
        - Activate a sprint after the other sprint is completed with 2-week schedule
        - Move a random number of issues in the backlog to the board
        - Randomly update an issue in the board and in the backlog
        - Randonly create a comment in an issue in the board and in the backlog
        - Randomly follow and unfollow an issue
    - Visit other pages
        - Paginate through my work page
        - Paginate through my recent issues
        - Paginate through my followed issues
        - Project listing
        - Organisation page
        - Automations page
        - Notifications page
        - Activity logs
        - My account
        - My profile
        - My billing
        - My integrations
- After all users are done, repeat the process

## Usage

```shell
au4-crawler --config path/to/config.toml
```
## Config

```toml
token = "token"
base_url = "https://example.com/api"
project_id = "123"
issue_count = 10
```
