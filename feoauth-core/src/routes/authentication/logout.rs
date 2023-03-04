use axum::{
    extract::{Query, State},
    response::{IntoResponse, Response},
    Extension, Json,
};
use http::StatusCode;
use serde::{Deserialize, Serialize};

use crate::{server::ApiContext, utils::app_error::AppError};

#[derive(Deserialize)]
pub struct LogoutParams {}

#[derive(Serialize)]
pub struct LogoutResponse {}

pub async fn logout(
    State(context): State<ApiContext>,
    Json(body): Json<LogoutParams>,
) -> Result<Response, AppError> {
    Ok((StatusCode::OK, "logout").into_response())
}
