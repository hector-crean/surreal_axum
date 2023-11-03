use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::fmt;
use thiserror::Error;
use uuid::Uuid;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    SerdeJson(#[from] serde_json::Error),
    #[error(transparent)]
    SurrealDb(#[from] surrealdb::Error),
    #[error(transparent)]
    JsonWebtoken(#[from] jsonwebtoken::errors::Error),
}

// ApiError has to have the req_id to report to the client and implements IntoResponse.
pub type ApiResult<T> = Result<T, Error>;

// REST error response
impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        let status_code = match &self {
            Error::SerdeJson { .. } => StatusCode::BAD_REQUEST,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        };
        let body = Json(json!({
            "error": self.to_string()
        }));
        let mut response = (status_code, body).into_response();
        // Insert the real Error into the response - for the logger
        response.extensions_mut().insert(self);
        response
    }
}

// for sending serialized keys through gql extensions
pub const ERROR_SER_KEY: &str = "error_ser";
