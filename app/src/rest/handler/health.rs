use axum::Json;
use axum::http::StatusCode;
use serde_json::{Value, json};

pub async fn health() -> (StatusCode, Json<Value>) {
    let body = json!({"status": "up"});
    (StatusCode::OK, Json(body))
}

pub async fn ready() -> StatusCode {
    StatusCode::OK
}
