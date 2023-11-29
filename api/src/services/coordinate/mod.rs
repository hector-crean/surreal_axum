use axum::extract::Path;
use axum::extract::State;
use axum::Json;

use models::user::{CreateUser, User};
use serde::Deserialize;
use serde::Serialize;
use surrealdb::engine::remote::ws::Client;
use surrealdb::Surreal;

use crate::error::ApiError;
use crate::AppState;

use models::coordinate_system::Coordinate;

const COORDINATE_TABLE: &str = "coordinate";

pub async fn create(
    State(AppState { db, .. }): State<AppState>,
    Json(data): Json<Coordinate>,
) -> Result<Json<Vec<Coordinate>>, ApiError> {
    let resp = db.client.create(COORDINATE_TABLE).content(data).await?;
    Ok(Json(resp))
}

pub async fn read(
    State(AppState { db, .. }): State<AppState>,
    id: Path<String>,
) -> Result<Json<Option<Coordinate>>, ApiError> {
    let resp = db.client.select((COORDINATE_TABLE, &*id)).await?;
    Ok(Json(resp))
}

pub async fn list(
    State(AppState { db, .. }): State<AppState>,
) -> Result<Json<Vec<Coordinate>>, ApiError> {
    let resp = db.client.select(COORDINATE_TABLE).await?;
    Ok(Json(resp))
}
