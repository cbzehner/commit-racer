use rocket::Route;

mod api;
mod auth;
mod health_check;
mod hello;
mod index;

pub(crate) use api::slack_api_routes;
pub(crate) use auth::slack_auth_routes;
use health_check::health_check as health_check_route;
use hello::hello as hello_route;
use index::index as index_route;

pub(crate) fn base_routes() -> Vec<Route> {
    return rocket::routes![hello_route, health_check_route, index_route];
}
