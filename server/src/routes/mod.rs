use axum::Router;

use crate::server::ApiContext;

pub fn router(context: ApiContext) -> Router {
    Router::new().with_state(context)
}
