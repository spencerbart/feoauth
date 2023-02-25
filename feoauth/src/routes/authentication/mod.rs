use axum::{
    middleware,
    routing::{get, post},
    Router,
};

use crate::{server::ApiContext, utils::middleware::is_user};

mod authorize;
mod callback;
mod logout;
mod reauthenticate;
mod signup;
mod token;

pub fn router() -> Router<ApiContext> {
    Router::new().nest(
        "/authentication",
        Router::new()
            .merge(
                Router::new()
                    .route("/authorize", get(authorize::external_provider_redirect))
                    .route(
                        "/callback",
                        post(callback::external_provider_callback)
                            .get(callback::external_provider_callback),
                    )
                    .route("/signup", post(signup::signup))
                    .route("/token", post(token::token)),
            )
            .merge(
                Router::new()
                    .route("/logout", post(logout::logout))
                    .route("/reauthenticate", get(reauthenticate::reauthenticate))
                    .route_layer(middleware::from_fn(is_user)),
            ),
    )
}
