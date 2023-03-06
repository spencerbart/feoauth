use std::{net::SocketAddr, sync::Arc};

use aws_sdk_dynamodb as dynamodb;
use axum::{
    error_handling::HandleErrorLayer,
    http::{
        header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
        Method, StatusCode,
    },
    BoxError, Router,
};
use tower::ServiceBuilder;
use tower_http::{compression::CompressionLayer, cors::CorsLayer, trace::TraceLayer};
use tracing_subscriber::{
    filter::{EnvFilter, LevelFilter},
    fmt,
    prelude::*,
};

use crate::{config::Config, routes};

#[derive(Clone)]
pub struct ApiContext {
    pub config: Arc<Config>,
    pub dynamodb_client: Arc<dynamodb::Client>,
}

pub async fn run() {
    // load standard env variables
    let config = envy::from_env::<Config>().unwrap();

    // load aws credentials from env variables
    let aws_config = aws_config::load_from_env().await;
    // creates a dynamodb client
    let dynamodb_client = dynamodb::Client::new(&aws_config);

    // loads tracing filter from env variable
    let filter = EnvFilter::builder()
        .with_default_directive(LevelFilter::ERROR.into())
        .from_env_lossy();

    // initializes tracing
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(filter)
        .init();

    // initializes cors
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::DELETE])
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);

    // creates a socket address to expose the api
    let addr = SocketAddr::from(([0, 0, 0, 0], config.port));

    // loads the api context to be used throughout the applicaiton
    let context = ApiContext {
        config: Arc::new(config),
        dynamodb_client: Arc::new(dynamodb_client),
    };

    // creates the api router and applies the middlewares
    let api_router = Router::new().nest("/v1", routes::router(context)).layer(
        ServiceBuilder::new()
            .layer(CompressionLayer::new())
            .layer(TraceLayer::new_for_http())
            .layer(cors)
            .layer(HandleErrorLayer::new(|_: BoxError| async {
                StatusCode::REQUEST_TIMEOUT
            }))
            .timeout(std::time::Duration::from_secs(30)),
    );

    println!("All configured, starting server at {}", addr);
    // starts the server
    axum::Server::bind(&addr)
        .serve(api_router.into_make_service())
        .await
        .unwrap();
}
