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

const USER_TABLE: &str = "user";

pub async fn create(
    State(AppState { db, .. }): State<AppState>,
    id: Path<String>,
    Json(data): Json<CreateUser>,
) -> Result<Json<Option<User>>, ApiError> {
    let user = db.client.create((USER_TABLE, &*id)).content(data).await?;
    Ok(Json(user))
}

pub async fn read(
    State(AppState { db, .. }): State<AppState>,
    id: Path<String>,
) -> Result<Json<Option<User>>, ApiError> {
    let user = db.client.select((USER_TABLE, &*id)).await?;
    Ok(Json(user))
}

pub async fn update(
    State(AppState { db, .. }): State<AppState>,
    id: Path<String>,
    Json(user): Json<User>,
) -> Result<Json<Option<User>>, ApiError> {
    let user = db.client.update((USER_TABLE, &*id)).content(user).await?;
    Ok(Json(user))
}

pub async fn delete(
    State(AppState { db, .. }): State<AppState>,
    id: Path<String>,
) -> Result<Json<Option<User>>, ApiError> {
    let user = db.client.delete((USER_TABLE, &*id)).await?;
    Ok(Json(user))
}

pub async fn list(
    State(AppState { db, .. }): State<AppState>,
) -> Result<Json<Vec<User>>, ApiError> {
    let people = db.client.select(USER_TABLE).await?;
    Ok(Json(people))
}
