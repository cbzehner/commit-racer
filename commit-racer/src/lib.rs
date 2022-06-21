use rocket::{Build, Rocket};

use rocket_dyn_templates::Template;
use rocket_oauth2::OAuth2;

use self::config::configuration;
use self::routes::{base_routes, slack_api_routes, slack_auth_routes};

mod config;
mod models;
mod routes;

pub fn rocket() -> Rocket<Build> {
    rocket::custom(configuration())
        .attach(Template::fairing())
        .attach(OAuth2::<models::slack::AuthResponse>::fairing("slack"))
        .mount("/", base_routes())
        .mount("/auth/slack", slack_auth_routes())
        .mount("/api/slack", slack_api_routes())
}
