use actix_web::{get, HttpResponse};
use serde_json::json;

#[get("/healthcheck")]
async fn healthcheck() -> HttpResponse{
    HttpResponse::Ok().json(
        json!({
            "status": "success"
        })
    )
}