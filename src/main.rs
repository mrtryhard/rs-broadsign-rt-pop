#[macro_use]
extern crate log;
mod broadsign;

use actix_web::{middleware, web, App, HttpResponse, HttpServer};
use broadsign::real_time_pop_request::RealTimePopRequest;

// We keep authentication at its simplest form, but you could
// return the api user informations through a Result<UserIdentity> mechanism.
pub fn authenticate(api_key: &String) -> bool {
    api_key == "some_secure_api_key"
}

pub async fn status_get() -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub async fn pop_post(pop_data: web::Json<RealTimePopRequest>) -> HttpResponse {
    let pop_data: RealTimePopRequest = pop_data.into_inner();

    debug!("Received pop submission:\n{:?}", pop_data);

    if !authenticate(&pop_data.api_key) {
        error!("Pop submission refused for api key '{}'", &pop_data.api_key);
        return HttpResponse::Unauthorized().finish();
    }

    HttpResponse::Ok().finish()
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    if !std::env::vars().any(|(k, _)| k == "RUST_LOG") {
        std::env::set_var("RUST_LOG", "info");
    }
    env_logger::init();

    info!("=== Starting Real-Time Pop Service ===");

    HttpServer::new(move || {
        App::new().wrap(middleware::Logger::default()).service(
            web::scope("")
                .route("/status", web::get().to(status_get))
                .route("/pop", web::post().to(pop_post)),
        )
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}

#[cfg(test)]
mod tests_endpoint_status {
    use super::*;
    use actix_web::http;

    #[actix_rt::test]
    async fn given_everything_is_running_status_returns_200_ok() {
        let resp = status_get().await;

        assert_eq!(resp.status(), http::StatusCode::OK);
    }
}

#[cfg(test)]
mod tests_endpoint_pop {
    use super::*;
    use actix_web::{http, web};
    use broadsign::real_time_pop_request::{RealTimePopEntry, RealTimePopRequest};
    use serde_json::json;

    fn make_valid_pop_request() -> RealTimePopRequest {
        RealTimePopRequest {
            api_key: "some_secure_api_key".to_owned(),
            player_id: 123456,
            pops: vec![RealTimePopEntry {
                display_unit_id: 123,
                frame_id: 124,
                active_screens_count: 2,
                ad_copy_id: 56467,
                campaign_id: 61000,
                schedule_id: 61001,
                impressions: 675,
                interactions: 0,
                end_time: chrono::NaiveDate::from_ymd(2017, 11, 23).and_hms_milli(13, 27, 12, 500),
                duration_ms: 12996,
                service_name: "bmb".to_owned(),
                service_value: "701".to_owned(),
                extra_data: json!(""),
            }],
        }
    }

    #[actix_rt::test]
    async fn given_a_valid_pop_and_healthy_server_respond_ok() {
        let resp = pop_post(web::Json(make_valid_pop_request())).await;

        assert_eq!(resp.status(), http::StatusCode::OK);
    }

    #[actix_rt::test]
    async fn given_an_invalid_api_key_server_responds_401_unauthorized() {
        let mut request = make_valid_pop_request();
        request.api_key = "some_invalid_api_key".to_owned();

        let resp = pop_post(web::Json(request)).await;

        assert_eq!(resp.status(), http::StatusCode::UNAUTHORIZED);
    }
}
