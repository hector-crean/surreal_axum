use std::{
    io::Read,
    path::{Path, PathBuf},
};

use crate::{services::s3::S3Error, AppState};
use axum::{
    extract::State,
    response::{IntoResponse, Json, Response},
};
use axum_typed_multipart::{FieldData, TryFromMultipart, TypedMultipart};
use chrono::{DateTime, NaiveDate, Utc};
use http::StatusCode;
use models::video::{CreateVideoFromFilePath, Video};
use tempfile::{NamedTempFile, PersistError};
use tokio::sync::broadcast::error::SendError;
use uuid::Uuid;

#[derive(thiserror::Error, Debug)]
pub enum CreateVideoError {
    #[error(transparent)]
    S3Error(#[from] S3Error),
    #[error(transparent)]
    PersistError(#[from] PersistError),
}

impl IntoResponse for CreateVideoError {
    fn into_response(self) -> axum::response::Response {
        match self {
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Unkown Error".to_string(),
            )
                .into_response(),
        }
    }
}

pub async fn create_video(
    State(state): State<AppState>,
    Json(CreateVideoFromFilePath {
        title,
        description,
        path_buf,
    }): Json<CreateVideoFromFilePath>,
) -> Result<Json<Video>, CreateVideoError> {
    let mut trans = state.pool.begin().await?;

    let s3_key = Uuid::new_v4().to_string();

    let s3_url = state
        .bucket
        .upload_object_from_file_path(path_buf, format!("{}", &s3_key).as_str())
        .await?;

    let video_id = Uuid::new_v4();

    let dt = chrono::offset::Utc::now();

    let video = query_as!(
        Video,
        r#"INSERT INTO video (video_id, title, description, s3_key, s3_url, updated_at, created_at) VALUES ($1, $2, $3, $4, $5, $6, $7) RETURNING *"#,
        video_id,
        title,
        description,
        s3_key,
        s3_url,
        dt,
        dt
    )
    .fetch_one(&mut *trans)
    .await?;

    // Commit the transaction since both operations succeeded
    trans.commit().await?;

    Ok(Json(video))
}

#[derive(TryFromMultipart)]
pub struct CreateVideoFromFile {
    pub title: String,
    pub description: Option<String>,
    // The `unlimited arguments` means that this field will be limited to the
    // total size of the request body. If you want to limit the size of this
    // field to a specific value you can also specify a limit in bytes, like
    // '5MiB' or '1GiB'.
    #[form_data(limit = "unlimited")]
    pub file: FieldData<NamedTempFile>,
}

pub async fn create_video_multipart(
    State(state): State<AppState>,
    TypedMultipart(CreateVideoFromFile {
        title,
        description,
        file,
    }): TypedMultipart<CreateVideoFromFile>,
) -> Result<Json<Video>, CreateVideoError> {
    // let file = match tokio::fs::File::open("Cargo.toml").await {
    //     Ok(file) => file,
    //     Err(err) => return Err((StatusCode::NOT_FOUND, format!("File not found: {}", err))),
    // };

    let mut trans = state.pool.begin().await?;

    let (_, temp_path) = file.contents.into_parts();

    let s3_key = Uuid::new_v4().to_string();

    let s3_url = state
        .bucket
        .upload_object_from_file_path(temp_path, format!("{}", &s3_key).as_str())
        .await?;

    let video_id = Uuid::new_v4();

    let dt = chrono::offset::Utc::now();

    let video = query_as!(
            Video,
            r#"INSERT INTO video (video_id, title, description, s3_key, s3_url, updated_at, created_at) VALUES ($1, $2, $3, $4, $5, $6, $7) RETURNING *"#,
            video_id,
            title,
            description,
            s3_key,
            s3_url,
            dt,
            dt
        )
        .fetch_one(&mut *trans)
        .await?;

    // Commit the transaction since both operations succeeded
    trans.commit().await?;

    Ok(Json(video))
}
