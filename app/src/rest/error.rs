use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::Serialize;

#[allow(dead_code)]
#[derive(Debug, Serialize)]
pub struct ErrorBody {
    pub error: String,
    pub code: u16,
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum AppError {
    NotFound(String),
    BadRequest(String),
    Internal(anyhow::Error),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),
            AppError::Internal(err) => {
                tracing::error!("Erro interno: {err:#}");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Erro interno do servidor".to_string(),
                )
            }
        };

        let body = Json(ErrorBody {
            error: message,
            code: status.as_u16(),
        });

        (status, body).into_response()
    }
}

impl From<anyhow::Error> for AppError {
    fn from(err: anyhow::Error) -> Self {
        AppError::Internal(err)
    }
}
