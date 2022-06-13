use rocket::Route;

mod health_check;
mod hello;
mod index;

use health_check::health_check as health_check_route;
use hello::hello as hello_route;
use index::index as index_route;

pub(crate) fn base_routes() -> Vec<Route> {
    return rocket::routes![hello_route, health_check_route, index_route];
}
