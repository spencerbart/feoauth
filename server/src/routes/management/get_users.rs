use axum::{
    extract::State,
    response::{IntoResponse, Response},
};
use http::StatusCode;

use crate::{server::ApiContext, utils::app_error::AppError};

pub async fn get_users(State(context): State<ApiContext>) -> Result<Response, AppError> {
    Ok((StatusCode::OK, "get users").into_response())
}
