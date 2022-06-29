mod github;
mod slack;

pub(crate) use github::auth_routes as github_auth_routes;
pub(crate) use slack::auth_routes as slack_auth_routes;
