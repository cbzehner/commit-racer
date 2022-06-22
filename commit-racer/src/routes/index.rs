use rocket_dyn_templates::{context, Template};

use crate::models::User;

#[rocket::get("/", rank = 2)]
pub(crate) fn logged_out() -> Template {
    println!("Logged out, no user found!");
    Template::render("logged-out", context! {})
}

#[rocket::get("/")]
pub(crate) fn logged_in(user: User) -> Template {
    println!("Logged in! Found user: {:?}", user);
    Template::render("logged-in", context! { user })
}

// #[cfg(test)]
// mod test {
//     use crate::rocket;
//     use rocket::http::Status;
//     use rocket::serde::json::serde_json;

//     use super::HealthCheckResponse;

//     #[test]
//     fn test_hello() {
//         use rocket::local::blocking::Client;

//         let client = Client::tracked(rocket()).unwrap();
//         let response = client.get("/hello").dispatch();
//         assert_eq!(response.status(), Status::Ok);
//         assert_eq!(response.into_string(), Some("Hello, world!".into()));
//     }

//     #[test]
//     fn test_health_check() {
//         use rocket::local::blocking::Client;

//         let client = Client::tracked(rocket()).unwrap();
//         let response = client.get("/healthz").dispatch();
//         assert_eq!(response.status(), Status::Ok);
//         assert_eq!(
//             serde_json::from_str::<HealthCheckResponse>(&response.into_string().unwrap()).unwrap(),
//             HealthCheckResponse {
//                 status: "ok".into()
//             }
//         );
//     }
// }
