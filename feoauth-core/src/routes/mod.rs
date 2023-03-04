use axum::Router;

use crate::server::ApiContext;

mod authentication;
mod management;

pub fn router(context: ApiContext) -> Router {
    Router::new()
        .merge(management::router())
        .merge(authentication::router())
        .with_state(context)
}
