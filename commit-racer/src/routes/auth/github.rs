use rocket::http::{Cookie, CookieJar};
use rocket::response::Redirect;
use rocket::Route;
use rocket_db_pools::sqlx::PgPool;
use rocket_oauth2::{OAuth2, TokenResponse};

use crate::db::DB;
use crate::models::github::AuthResponse;
use crate::models::User;

pub(crate) fn auth_routes() -> Vec<Route> {
    return rocket::routes![login, callback];
}

#[rocket::get("/login")]
fn login(oauth2: OAuth2<AuthResponse>, cookies: &CookieJar<'_>) -> Redirect {
    // TODO: Add scopes...
    oauth2.get_redirect(cookies, &[]).unwrap()
}

#[rocket::get("/callback")]
async fn callback(token: TokenResponse<AuthResponse>, user: User, pool: &DB) -> Redirect {
    let token = token.as_value();

    // Write the token authentication response to the database
    // Connect the GitHub user to the existing User model
    // Redirect back to the main page

    Redirect::to("/")
}

#[cfg(test)]
mod test {
    use rocket::http::Status;
    use url::Url;

    use crate::rocket;

    // TODO: Re-write this test for GitHub
    #[tokio::test]
    async fn test_slack_login() {
        use rocket::local::asynchronous::Client;

        // arrange
        let client = Client::tracked(rocket()).await.unwrap();

        // act
        let response = client.get("/auth/slack/login").dispatch().await;

        // assert
        assert_eq!(response.status(), Status::SeeOther);
        assert!(response.cookies().get("rocket_oauth2_state").is_some());

        let mut values = response.headers().get("Location");
        let slack_auth_url = Url::parse(values.next().unwrap()).unwrap();
        assert_eq!(slack_auth_url.domain(), Some("slack.com"));
        assert!(slack_auth_url
            .query_pairs()
            .find(|(k, _)| k == "redirect_uri")
            .is_some());
    }

    // #[tokio::test]
    // async fn test_slack_callback() {
    //     use rocket::local::asynchronous::Client;

    //     // arrange
    //     let client = Client::tracked(rocket()).await.unwrap();
    //     let oauth2_state_value = "12345-ABCDE";
    //     let _ = client.post("/").cookie(Cookie::new("rocket_oauth2_state", oauth2_state_value)).dispatch().await;
    //     // TODO: Figure out how to set the cookie on the Client...
    //     println!("{:?}", client.cookies());
    //     // act
    //     let callback_url = format!("/auth/slack/callback?code=498993223299.3451076857142.43b4fa0c26d04d017491cc41dccdbf44ff2e5802a1352e74274410a26c3b62ff&state={oauth2_state_value}");
    //     let response = client.get(callback_url).dispatch().await;

    //     // assert
    //     assert_eq!(response.status(), Status::SeeOther);
    //     assert!(response.cookies().get("rocket_oauth2_state").is_none())
    // }
}
