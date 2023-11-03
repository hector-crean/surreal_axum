use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use geo_types::Point;

#[derive(Debug, Deserialize, Serialize)]
pub struct VideoComment {
    pub comment_id: Uuid,
    pub user_id: Option<Uuid>,
    pub video_id: Option<Uuid>,
    pub start_time: f32,
    pub end_time: Option<f32>,
    // pub coordinates: Point2D,
    pub coordinates: Point,
    pub comment_text: String,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateVideoComment {
    pub video_id: Option<Uuid>,
    pub user_id: Option<Uuid>,
    pub comment_text: String,
    pub coordinates: Point,
    pub start_time: f32,
    pub end_time: Option<f32>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetVideoComment {
    pub comment_id: Uuid,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UpdateVideoComment {
    pub comment_id: Uuid,
    pub markdown: String,
}
