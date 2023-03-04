use axum::{
    extract::{Query, State},
    response::{IntoResponse, Response},
    Extension,
};
use http::StatusCode;
use serde::Deserialize;

use crate::{server::ApiContext, utils::app_error::AppError};

#[derive(Deserialize)]
pub struct GetUsersParams {
    pub page: Option<i64>,
    pub per_page: Option<i64>,
}

pub async fn get_users(
    State(context): State<ApiContext>,
    Query(query): Query<GetUsersParams>,
    Extension(user_id): Extension<String>,
) -> Result<Response, AppError> {
    println!("user_id: {}", user_id);
    Ok((StatusCode::OK, "get users").into_response())
}
