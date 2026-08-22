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

#[derive(Serialize, Debug)]
pub struct ValidationErrorResponse {
    pub erro: String,
    pub campos: Vec<FieldError>,
}

#[derive(Serialize, Debug)]
pub struct FieldError {
    pub campo: String,
    pub codigo: String,
    pub mensagem: String,
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum AppError {
    NotFound(String),
    BadRequest(String),
    Validation(ValidationErrorResponse),
    Internal(anyhow::Error),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            AppError::NotFound(msg) => {
                let body = Json(ErrorBody {
                    error: msg,
                    code: StatusCode::NOT_FOUND.as_u16(),
                });
                (StatusCode::NOT_FOUND, body).into_response()
            }
            AppError::BadRequest(msg) => {
                let body = Json(ErrorBody {
                    error: msg,
                    code: StatusCode::BAD_REQUEST.as_u16(),
                });
                (StatusCode::BAD_REQUEST, body).into_response()
            }
            AppError::Validation(err) => (StatusCode::BAD_REQUEST, Json(err)).into_response(),
            AppError::Internal(err) => {
                tracing::error!("Erro interno: {err:#}");
                let body = Json(ErrorBody {
                    error: "Erro interno do servidor".to_string(),
                    code: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                });
                (StatusCode::INTERNAL_SERVER_ERROR, body).into_response()
            }
        }
    }
}

impl From<anyhow::Error> for AppError {
    fn from(err: anyhow::Error) -> Self {
        AppError::Internal(err)
    }
}

impl From<validator::ValidationErrors> for AppError {
    fn from(errors: validator::ValidationErrors) -> Self {
        let campos = errors
            .field_errors()
            .into_iter()
            .map(|(field, errs)| {
                let err = &errs[0];
                FieldError {
                    campo: field.to_string(),
                    codigo: err.code.to_string(),
                    mensagem: err
                        .message
                        .as_ref()
                        .map(|m| m.to_string())
                        .unwrap_or_else(|| err.code.to_string()),
                }
            })
            .collect();

        AppError::Validation(ValidationErrorResponse {
            erro: "validacao_falhou".to_string(),
            campos,
        })
    }
}
