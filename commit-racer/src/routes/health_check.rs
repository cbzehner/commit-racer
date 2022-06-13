use rocket::serde::json::Json;

#[derive(Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub(crate) struct HealthCheckResponse {
    status: String,
}

#[rocket::get("/healthz")]
pub(crate) fn health_check() -> Json<HealthCheckResponse> {
    Json(HealthCheckResponse {
        status: "ok".into(),
    })
}

#[cfg(test)]
mod test {
    use crate::rocket;
    use rocket::http::Status;
    use rocket::serde::json::serde_json;

    use super::HealthCheckResponse;

    #[test]
    fn test_health_check() {
        use rocket::local::blocking::Client;

        let client = Client::tracked(rocket()).unwrap();
        let response = client.get("/healthz").dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(
            serde_json::from_str::<HealthCheckResponse>(&response.into_string().unwrap()).unwrap(),
            HealthCheckResponse {
                status: "ok".into()
            }
        );
    }
}
