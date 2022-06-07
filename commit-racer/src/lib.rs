use dotenv;
use rocket::{Build, Rocket};

use routes::base_routes;

mod routes;

pub fn rocket() -> Rocket<Build> {
    dotenv::dotenv().ok();
    rocket::build().mount("/", base_routes())
}
