pub mod auth;
pub mod db;
pub mod error;
pub mod services;

use crate::services::open_ai;
use auth::auth;
use axum::{
    extract::DefaultBodyLimit,
    middleware,
    routing::{delete, get, post, put},
    Router,
};
pub use db::Database;
use http::{header::CONTENT_TYPE, Method};
use models::random::Random;
use open_ai_client::OpenAiClient;
use services::{s3::S3Bucket, user};
use tower_http::{
    cors::{Any, CorsLayer},
    trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer},
};
use tracing::Level;

#[derive(Clone)]
pub struct AppState {
    db: Database,
    random: Random,
    open_ai_client: OpenAiClient,
    bucket: S3Bucket,
}

impl AppState {
    pub fn new(
        db: Database,
        random: Random,
        open_ai_client: OpenAiClient,
        bucket: S3Bucket,
    ) -> Self {
        Self {
            db,
            random,
            open_ai_client,
            bucket,
        }
    }

    pub async fn router(self) -> Result<axum::Router, error::ApiError> {
        let http_trace_layer = TraceLayer::new_for_http()
            .make_span_with(
                DefaultMakeSpan::new()
                    .level(Level::INFO)
                    .level(Level::DEBUG),
            )
            .on_response(
                DefaultOnResponse::new()
                    .level(Level::INFO)
                    .level(Level::DEBUG),
            );

        let cors_layer = CorsLayer::new()
            // allow `GET` and `POST` when accessing the resource
            .allow_methods([Method::GET, Method::POST])
            // allow requests from any origin
            .allow_origin(Any)
            .allow_headers([CONTENT_TYPE]);

        let router = Router::new()
            .route(
                "/user/:id",
                post(user::create)
                    .get(user::read)
                    .put(user::update)
                    .delete(user::delete),
            )
            .route("/user", get(user::list))
            .route("/user/:id/chat", post(open_ai::create_chat))
            .route("/geometry/:id", get(user::read))
            .route("/geometry", get(video::list).post(video::create))
            .with_state(self);

        let api = Router::new()
            .nest("/v1/api", router)
            .layer(DefaultBodyLimit::max(1024 * 1024 * 1024))
            .layer(CorsLayer::permissive())
            .layer(http_trace_layer);
        // .route_layer(middleware::from_fn(auth));
        Ok(api)
    }
}
