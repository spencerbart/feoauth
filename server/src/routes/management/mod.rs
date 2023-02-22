use axum::{middleware, routing::get, Router};

use crate::{server::ApiContext, utils::middleware::is_user};

mod get_users;

pub fn router() -> Router<ApiContext> {
    let router = Router::new()
        .route("/users", get(get_users::get_users))
        .route_layer(middleware::from_fn(is_user));

    Router::new().nest("/stripe", router)
}
