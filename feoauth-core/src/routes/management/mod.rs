use axum::{
    middleware,
    routing::{get, post},
    Router,
};

use crate::{server::ApiContext, utils::middleware::is_admin};

mod create_user;
mod get_audit_logs;
mod get_users;
mod invite_user;

pub fn router() -> Router<ApiContext> {
    Router::new().nest(
        "/management",
        Router::new()
            .route("/audit-logs", get(get_audit_logs::get_audit_logs))
            .route(
                "/users",
                post(create_user::create_user).get(get_users::get_users),
            )
            .route("/invite", post(invite_user::invite_user))
            .route_layer(middleware::from_fn(is_admin)),
    )
}
