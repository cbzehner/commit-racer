use dotenv;
use rocket::{Build, Rocket};

use rocket_dyn_templates::Template;
use routes::base_routes;

mod routes;

pub fn rocket() -> Rocket<Build> {
    dotenv::dotenv().ok();
    rocket::build()
        .attach(Template::fairing())
        .mount("/", base_routes())
}
