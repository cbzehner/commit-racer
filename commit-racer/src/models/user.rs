use rocket::request::{self, FromRequest, Outcome, Request};
use rocket_db_pools::sqlx::{pool::PoolConnection, Postgres};
use rocket_db_pools::Connection;
use uuid::Uuid;

use crate::db::DB;

#[derive(Debug, serde::Serialize)]
pub(crate) struct User {
    pub(crate) id: Uuid,
    github_user_id: Option<String>,
    slack_user_id: String,
}

impl User {
    async fn get_by_id(
        mut conn: PoolConnection<Postgres>,
        id: Uuid,
    ) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_file_as!(User, "../db/queries/query_user_by_id.sql", Some(id))
            .fetch_optional(&mut conn)
            .await
    }

    pub(crate) async fn get_or_create_by_slack_id(
        mut conn: PoolConnection<Postgres>,
        slack_user_id: String,
    ) -> Result<Self, sqlx::Error> {
        sqlx::query_file_as!(
            Self,
            "../db/queries/upsert_user_by_slack_user_id.sql",
            Some(slack_user_id),
        )
        .fetch_one(&mut conn)
        .await
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for User {
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        // TODO: Refactor to use Request-Local State (https://api.rocket.rs/v0.5-rc/rocket/request/trait.FromRequest.html#request-local-state)
        let conn = req.guard::<Connection<DB>>().await.succeeded();
        let user_id = req
            .cookies()
            .get("user_id")
            .and_then(|cookie| cookie.value().parse::<Uuid>().ok());
        match (user_id, conn) {
            (None, _) => Outcome::Forward(()),
            (Some(user_id), Some(conn)) => {
                match User::get_by_id(conn.into_inner(), user_id)
                    .await
                    .ok()
                    .flatten()
                {
                    Some(user) => Outcome::Success(user),
                    None => Outcome::Forward(()),
                }
            }
            _ => Outcome::Forward(()),
        }
    }
}
