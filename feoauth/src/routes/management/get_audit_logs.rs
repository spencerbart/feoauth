use axum::{
    extract::{Query, State},
    response::{IntoResponse, Response},
    Extension,
};
use http::StatusCode;
use serde::Deserialize;

use crate::{server::ApiContext, utils::app_error::AppError};

#[derive(Deserialize)]
pub struct GetAuditLogsParams {
    pub page: Option<i64>,
    pub per_page: Option<i64>,
}

pub async fn get_audit_logs(
    State(context): State<ApiContext>,
    Query(query): Query<GetAuditLogsParams>,
    Extension(user_id): Extension<String>,
) -> Result<Response, AppError> {
    Ok((StatusCode::OK, "get audit logs").into_response())
}
