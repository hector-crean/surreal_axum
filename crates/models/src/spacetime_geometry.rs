use chrono::{DateTime, Utc};
use core::time::Duration;
use geo_types::Point;
use serde::{Deserialize, Serialize};
use surrealdb::sql;

use crate::user::User;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SpacetimeGeometry {
    pub geometry: sql::Geometry,
    pub timestamp: f32,
    pub duration: sql::Duration,
    pub title: String,
    pub text_body: String,
    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
}

#[derive(Serialize, Deserialize)]
pub struct CreateSpacetimeGeometry {
    pub author: Option<User>,
    pub geometry: geo::Point,
    pub timestamp: f32,
    pub duration: sql::Duration,
    pub title: String,
    pub text_body: String,
}
