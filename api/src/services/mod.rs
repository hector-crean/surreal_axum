pub mod open_ai;
use axum::response::{IntoResponse, Response};
use http::StatusCode;

#[derive(thiserror::Error, Debug)]
pub enum ApiError {
    #[error(transparent)]
    ChronoParseError(#[from] chrono::ParseError),
    #[error("Unknown Error")]
    Unknown,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        match self {
            ApiError::ChronoParseError(err) => {
                (StatusCode::INTERNAL_SERVER_ERROR, format!("{}", err)).into_response()
            }
            ApiError::Unknown => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Unkown Error".to_string(),
            )
                .into_response(),
        }
    }
}
