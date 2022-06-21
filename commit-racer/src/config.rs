use dotenvy;
use rocket::figment::{self, Figment};
use rocket::Config;

// Customized configuration for this application.
pub(crate) fn configuration() -> Figment {
    load_environment();

    Config::figment()
        .merge(template_location())
        .merge(slack_oauth_config())
}

// Load dotenv files using the standard precedence ordering.
fn load_environment() -> () {
    // https://github.com/bkeepers/dotenv#what-other-env-files-can-i-use
    dotenvy::from_filename(paths(".env")).ok();
    dotenvy::from_filename(paths(".env.production")).ok();
    dotenvy::from_filename(paths(".env.test")).ok();
    dotenvy::from_filename(paths(".env.development")).ok();
    dotenvy::from_filename(paths(".env")).ok();
    dotenvy::from_filename(paths(".env.production.local")).ok();
    dotenvy::from_filename(paths(".env.test.local")).ok();
    dotenvy::from_filename(paths(".env.development.local")).ok();
}

// Location of Tera templates
fn template_location() -> Figment {
    Figment::from(("template_dir", paths("templates/")))
}

// Create Slack OAuth2 configuration from local environment.
fn slack_oauth_config() -> Figment {
    let auth_uri = dotenvy::var("ROCKET_OAUTH_SLACK_AUTH_URI")
        .expect("Missing environment variable ROCKET_OAUTH_SLACK_AUTH_URI");
    let token_uri = dotenvy::var("ROCKET_OAUTH_SLACK_TOKEN_URI")
        .expect("Missing environment variable ROCKET_OAUTH_SLACK_TOKEN_URI");
    let client_id = dotenvy::var("ROCKET_OAUTH_SLACK_CLIENT_ID")
        .expect("Missing environment variable ROCKET_OAUTH_SLACK_CLIENT_ID");
    let client_secret = dotenvy::var("ROCKET_OAUTH_SLACK_CLIENT_SECRET")
        .expect("Missing environment variable ROCKET_OAUTH_SLACK_CLIENT_SECRET");
    let redirect_uri = dotenvy::var("ROCKET_OAUTH_SLACK_REDIRECT_URI")
        .expect("Missing environment variable ROCKET_OAUTH_SLACK_REDIRECT_URI");

    let values = figment::map! {
        "slack" => figment::map! {
            "auth_uri" => auth_uri,
            "token_uri" => token_uri,
            "client_id" => client_id,
            "client_secret" => client_secret,
            "redirect_uri" => redirect_uri,
        }
    };

    Figment::from(("oauth", values))
}

// Handle changes to the relative path between test and non-test builds.
// Non-test builds start path discovery from the workspace root.
// Test builds start path discovery from within the crate folder.
fn paths(path: &str) -> String {
    if cfg!(test) {
        format!("./{path}")
    } else {
        format!("./commit-racer/{path}")
    }
}
