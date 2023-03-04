use axum::{
    extract::{Query, State},
    response::{IntoResponse, Response},
    Extension,
};
use http::StatusCode;
use serde::Deserialize;

use crate::{server::ApiContext, utils::app_error::AppError};

pub async fn reauthenticate(State(context): State<ApiContext>) -> Result<Response, AppError> {
    Ok((StatusCode::OK, "reauthenticate").into_response())
}
