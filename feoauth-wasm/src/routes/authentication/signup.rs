use axum::{
    extract::{Query, State},
    response::{IntoResponse, Response},
    Extension, Json,
};
use http::StatusCode;
use serde::Deserialize;
use serde_json::Value;

use crate::{server::ApiContext, utils::app_error::AppError};

#[derive(Deserialize)]
pub struct SignupParams {
    pub email: Option<String>,
    pub phone: Option<String>,
    pub password: Option<String>,
    pub data: Option<Value>,
}

pub async fn signup(
    State(context): State<ApiContext>,
    Json(body): Json<SignupParams>,
) -> Result<Response, AppError> {
    Ok((StatusCode::OK, "signup").into_response())
}
