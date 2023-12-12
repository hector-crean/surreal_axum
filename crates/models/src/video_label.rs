use crate::{interval::Interval, record::Record, rich_text::RichText, style::Style, user::User};
use geojson::{de::deserialize_geometry, ser::serialize_geometry};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct VideoLabel {
    pub user_id: Option<Uuid>,
    pub video_id: Option<Uuid>,
    #[serde(
        serialize_with = "serialize_geometry",
        deserialize_with = "deserialize_geometry"
    )]
    pub geometry: geo::Geometry,
    pub geo_style: Style,
    pub interval: Interval<f32>,
    pub title: String,
    pub text_body: RichText,
}

pub type CreateVideoLabel = VideoLabel;

pub type VideoLabelRecord = Record<VideoLabel>;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetVideoLabel {
    pub label_id: Uuid,
}
