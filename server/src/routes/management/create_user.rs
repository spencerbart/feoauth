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
    #[validate(custom = "validate_password")]
    pub password: Option<String>,
}

pub async fn create_user(
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

    // if password doesn't exist, generate a random one
    let password = if let Some(password) = body.password {
        password
    } else {
        // generate random password
        let mut rng = rand::thread_rng();
        let password: String = std::iter::repeat(())
            .map(|()| rng.sample(rand::distributions::Alphanumeric))
            .map(char::from)
            .take(32)
            .collect();
        password
    };

    println!("user_id: {}", user_id);
    println!("password: {}", password);
    Ok((StatusCode::OK, "create user").into_response())
}

fn validate_password(password: &str) -> Result<(), ValidationError> {
    if password.len() < 8 {
        return Err(ValidationError::new(
            "Password must be at least 8 characters",
        ));
    }
    // make sure password contains at least one number
    if !password.chars().any(|c| c.is_numeric()) {
        return Err(ValidationError::new(
            "Password must contain at least one number",
        ));
    }
    // make sure password contains at least one uppercase letter
    if !password.chars().any(|c| c.is_uppercase()) {
        return Err(ValidationError::new(
            "Password must contain at least one uppercase letter",
        ));
    }
    // make sure password contains at least one lowercase letter
    if !password.chars().any(|c| c.is_lowercase()) {
        return Err(ValidationError::new(
            "Password must contain at least one lowercase letter",
        ));
    }
    Ok(())
}
