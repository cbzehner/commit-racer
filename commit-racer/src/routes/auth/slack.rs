use rocket::http::{Cookie, CookieJar, SameSite};
use rocket::response::Redirect;
use rocket::Route;
use rocket_oauth2::{OAuth2, TokenResponse};

use crate::models::slack::AuthResponse;

pub(crate) fn auth_routes() -> Vec<Route> {
    return rocket::routes![login, callback];
}

#[rocket::get("/login")]
fn login(oauth2: OAuth2<AuthResponse>, cookies: &CookieJar<'_>) -> Redirect {
    oauth2
        .get_redirect(cookies, &["chat:write", "commands", "users:read"])
        .unwrap()
}

#[rocket::get("/callback")]
fn callback(
    token: TokenResponse<AuthResponse>,
    cookies: &CookieJar<'_>,
    // pool: &DB,
) -> Redirect {
    // let value = token.as_value();
    // let response = Response(value.clone());
    // match response.insert(pool).await {
    //     Ok(_) => {
    //         cookies.add_private(
    //             Cookie::build("slack_token", token.access_token().to_string())
    //                 .http_only(true)
    //                 .same_site(SameSite::Strict)
    //                 .secure(true)
    //                 .finish(),
    //         );
    //         Redirect::to("/")
    //     }
    //     Err(err) => {
    //         println!("{:?}", err);
    //         Redirect::to("/")
    //     }
    // }
    cookies.add_private(
        Cookie::build("slack_token", token.access_token().to_string())
            .http_only(true)
            .same_site(SameSite::Strict)
            .secure(true)
            .finish(),
    );
    Redirect::to("/")
}

#[cfg(test)]
mod test {
    use rocket::http::Status;
    use url::Url;

    use crate::rocket;

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
