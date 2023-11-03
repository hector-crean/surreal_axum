pub mod db;
pub use db::Database;

pub mod error;
pub mod services;
use axum::{
    extract::DefaultBodyLimit,
    routing::{get, post},
    Router,
};
use http::{header::CONTENT_TYPE, Method};
use models::random::Random;

use tower_http::{
    cors::{Any, CorsLayer},
    trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer},
};
use tracing::Level;

#[derive(Clone)]
pub struct AppState {
    db: Database,
    random: Random,
}

impl AppState {
    pub fn new(db: Database, random: Random) -> Self {
        Self { db, random }
    }

    pub async fn router(self) -> error::ApiResult<axum::Router> {
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

        let router = Router::new().with_state(self);

        let api = Router::new()
            .nest("/:version/api", router)
            .layer(DefaultBodyLimit::max(1024 * 1024 * 1024))
            .layer(CorsLayer::permissive())
            .layer(http_trace_layer);
        Ok(api)
    }
}
