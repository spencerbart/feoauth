use axum::{
    extract::{Query, State},
    response::{IntoResponse, Response},
    Extension,
};
use http::StatusCode;
use serde::Deserialize;

use crate::{server::ApiContext, utils::app_error::AppError};

#[derive(Deserialize)]
pub struct ReauthenticateParams {
    pub page: Option<i64>,
    pub per_page: Option<i64>,
}

pub async fn reauthenticate(
    State(context): State<ApiContext>,
    Query(query): Query<ReauthenticateParams>,
) -> Result<Response, AppError> {
    Ok((StatusCode::OK, "get audit logs").into_response())
}
