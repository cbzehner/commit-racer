use dotenvy;
use rocket::figment::{self, Figment};
use rocket::Config;

// Customized configuration for this application.
pub(crate) fn configuration() -> Figment {
    load_environment();

    Config::figment()
        .merge(postgres_config())
        .merge(slack_oauth_config())
        .merge(template_location())
}

// Load dotenv files using the standard precedence ordering.
fn load_environment() -> () {
    // https://github.com/bkeepers/dotenv#what-other-env-files-can-i-use
    dotenvy::from_filename(".env").ok();
    dotenvy::from_filename(".env.production").ok();
    dotenvy::from_filename(".env.test").ok();
    dotenvy::from_filename(".env.development").ok();
    dotenvy::from_filename(".env").ok();
    dotenvy::from_filename(".env.production.local").ok();
    dotenvy::from_filename(".env.test.local").ok();
    dotenvy::from_filename(".env.development.local").ok();
}

// Location of Tera templates
fn template_location() -> Figment {
    Figment::from(("template_dir", "templates/"))
}

// Configure the PostgreSQL database
fn postgres_config() -> Figment {
    let user = dotenvy::var("PGUSER").expect("Missing environment variable PGUSER");
    let password = dotenvy::var("PGPASSWORD").unwrap_or("".to_owned());
    let host = dotenvy::var("PGHOST").expect("Missing environment variable PGHOST");
    let port = dotenvy::var("PGPORT").unwrap_or("5432".to_owned());
    let database = dotenvy::var("PGDATABASE").expect("Missing environment variable PGDATABASE");

    let postgres_url = format!("postgres://{user}:{password}@{host}:{port}/{database}");

    Figment::from((
        "databases.postgres",
        rocket_db_pools::Config {
            url: postgres_url,
            min_connections: Some(16), // TODO: Investigate increasing to 64.
            max_connections: 64, // TODO: Investigate supporting more connections. Only support 100 max connections locally. Ideally ~1024?
            connect_timeout: 5,  // Five seconds
            idle_timeout: Some(120), // One-hundred and twenty seconds
        },
    ))
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

    Figment::from((
        "oauth.slack",
        figment::map! {
            "auth_uri" => auth_uri,
            "token_uri" => token_uri,
            "client_id" => client_id,
            "client_secret" => client_secret,
            "redirect_uri" => redirect_uri,
        },
    ))
}
