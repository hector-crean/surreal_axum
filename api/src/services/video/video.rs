use crate::error::ApiError;
use crate::AppState;
use axum::extract::Path;
use axum::extract::State;
use axum::Json;
use models::record::Record;
use models::video_label::CreateVideoLabel;
use models::video_label::VideoLabel;

const VIDEO_TABLE: &str = "video_table";

pub async fn create(
    State(AppState { db, .. }): State<AppState>,
    Json(data): Json<CreateVideoLabel>,
) -> Result<Json<Vec<Record<VideoLabel>>>, ApiError> {
    let records = db.client.create(VIDEO_LABEL_TABLE).content(data).await?;
    Ok(Json(records))
}

pub async fn read(
    State(AppState { db, .. }): State<AppState>,
    id: Path<String>,
) -> Result<Json<Option<Record<VideoLabel>>>, ApiError> {
    let record = db.client.select((VIDEO_LABEL_TABLE, &*id)).await?;
    Ok(Json(record))
}

pub async fn list(
    State(AppState { db, .. }): State<AppState>,
) -> Result<Json<Vec<Record<VideoLabel>>>, ApiError> {
    let records = db.client.select(VIDEO_LABEL_TABLE).await?;
    Ok(Json(records))
}
