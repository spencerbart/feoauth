use axum::{
    response::{IntoResponse, Response},
    Json,
};
use http::StatusCode;

pub enum AppError {
    ReqwestError(reqwest::Error),
    Anyhow(anyhow::Error),
    Validate(validator::ValidationErrors),
    BadRequest(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            AppError::Anyhow(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(format!("Something went wrong: {}", e)),
            )
                .into_response(),
            AppError::Validate(e) => (
                StatusCode::BAD_REQUEST,
                Json(format!("Validation error: {}", e)),
            )
                .into_response(),
            AppError::ReqwestError(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(format!("Something went wrong: {}", e)),
            )
                .into_response(),
            AppError::BadRequest(e) => (StatusCode::BAD_REQUEST, Json(e)).into_response(),
        }
    }
}

impl From<validator::ValidationErrors> for AppError {
    fn from(err: validator::ValidationErrors) -> Self {
        Self::Validate(err)
    }
}

impl From<anyhow::Error> for AppError {
    fn from(err: anyhow::Error) -> Self {
        Self::Anyhow(err)
    }
}

impl From<reqwest::Error> for AppError {
    fn from(err: reqwest::Error) -> Self {
        Self::ReqwestError(err)
    }
}
