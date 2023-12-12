use axum_typed_multipart::{FieldData, TryFromMultipart, TypedMultipart};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::{fs::File, path::PathBuf};
use tempfile::NamedTempFile;
use uuid::Uuid;

use crate::video_label::VideoLabel;

#[derive(Debug, Deserialize, Serialize)]
pub struct Video {
    pub video_id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub s3_key: String,
    pub s3_url: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateVideoFromFilePath {
    pub title: String,
    pub description: Option<String>,
    pub path_buf: PathBuf,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetVideo {
    pub video_id: Uuid,
}
