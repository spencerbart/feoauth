use axum::{
    extract::{Query, State},
    response::{IntoResponse, Response},
    Extension, Json,
};
use http::StatusCode;
use serde::{Deserialize, Serialize};

use crate::{server::ApiContext, utils::app_error::AppError};

#[derive(Deserialize)]
pub struct TokenParams {
    pub grant_type: GrantType,
}

#[derive(Deserialize)]
#[serde(rename_all = "snake_case", deny_unknown_fields)]
pub enum GrantType {
    Password,
    RefreshToken,
    IdToken,
}

#[derive(Serialize)]
pub struct TokenResponse {}

pub async fn token(
    State(context): State<ApiContext>,
    Json(body): Json<TokenParams>,
) -> Result<Response, AppError> {
    Ok((StatusCode::OK, "token").into_response())
}
