use rocket::{Build, Rocket};

#[rocket::launch]
fn launch() -> Rocket<Build> {
    commit_racer::rocket()
}
