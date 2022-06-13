use dotenv;
use rocket::{Build, Config, Rocket};

use rocket_dyn_templates::Template;
use routes::base_routes;

mod routes;

pub fn rocket() -> Rocket<Build> {
    dotenv::dotenv().ok();

    let template_dir = if cfg!(test) {
        "./templates"
    } else {
        "./commit-racer/templates"
    };

    let configuration = Config::figment().merge(("template_dir", template_dir));

    rocket::custom(configuration)
        .attach(Template::fairing())
        .mount("/", base_routes())
}
