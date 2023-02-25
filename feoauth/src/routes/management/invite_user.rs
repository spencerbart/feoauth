use axum::{
    extract::State,
    response::{IntoResponse, Response},
    Extension, Json,
};
use http::StatusCode;
use rand::Rng;
use serde::Deserialize;
use validator::{Validate, ValidationError};

use crate::{server::ApiContext, utils::app_error::AppError};

#[derive(Deserialize, Validate)]
pub struct CreateUserParams {
    #[validate(email)]
    pub email: Option<String>,
    #[validate(phone)]
    pub phone: Option<String>,
}

pub async fn invite_user(
    State(context): State<ApiContext>,
    Extension(user_id): Extension<String>,
    Json(body): Json<CreateUserParams>,
) -> Result<Response, AppError> {
    body.validate()?;
    if body.email.is_none() && body.phone.is_none() {
        return Err(AppError::BadRequest(
            "email or phone is required".to_string(),
        ));
    }

    println!("user_id: {}", user_id);
    Ok((StatusCode::OK, "invited user").into_response())
}
