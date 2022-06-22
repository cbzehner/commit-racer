use rocket::http::{Cookie, CookieJar};
use rocket::response::Redirect;
use rocket::Route;
use rocket_db_pools::sqlx::PgPool;
use rocket_oauth2::{OAuth2, TokenResponse};

use crate::db::DB;
use crate::models::slack::AuthResponse;
use crate::models::User;

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
async fn callback(token: TokenResponse<AuthResponse>, pool: &DB, jar: &CookieJar<'_>) -> Redirect {
    let token = token.as_value();
    let slack_user_id = token["authed_user"]["id"].as_str().unwrap_or_default();

    let _ = match PgPool::acquire(pool).await {
        Ok(mut conn) => {
            let _ = sqlx::query_file!(
                "../db/queries/insert_slack_auth_response.sql",
                token["access_token"].as_str(),
                slack_user_id,
                token["bot_user_id"].as_str(),
                token["enterprise"].get("id").map(|v| v.as_str()).flatten(),
                token["enterprise"]
                    .get("name")
                    .map(|v| v.as_str())
                    .flatten(),
                token["scope"].as_str(),
                token["team"]["id"].as_str(),
                token["team"]["name"].as_str(),
                token,
            )
            .execute(&mut conn)
            .await;
            Ok(())
        }
        Err(error) => Err(error),
    };

    let result = match PgPool::acquire(pool).await {
        Ok(conn) => User::get_or_create_by_slack_id(conn, slack_user_id.to_owned()).await,
        Err(error) => Err(error),
    };

    match result {
        Ok(user) => jar.add(Cookie::new("user_id", user.id.to_string())),
        Err(err) => println!("{:?}", err),
    };
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
