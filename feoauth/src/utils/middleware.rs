use anyhow::Result;
use axum::{
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};

use super::auth::get_user_id;

pub async fn is_admin<B>(mut req: Request<B>, next: Next<B>) -> Result<Response, StatusCode> {
    let auth_header = req.headers().get(http::header::AUTHORIZATION);

    match get_user_id(auth_header) {
        Some(user_id) => {
            req.extensions_mut().insert(user_id);
            Ok(next.run(req).await)
        }
        None => Err(StatusCode::UNAUTHORIZED),
    }
}

pub async fn is_user<B>(mut req: Request<B>, next: Next<B>) -> Result<Response, StatusCode> {
    let auth_header = req.headers().get(http::header::AUTHORIZATION);

    match get_user_id(auth_header) {
        Some(user_id) => {
            req.extensions_mut().insert(user_id);
            Ok(next.run(req).await)
        }
        None => Err(StatusCode::UNAUTHORIZED),
    }
}
