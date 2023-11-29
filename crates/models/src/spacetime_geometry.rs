use super::geometry::GeoJson;
use crate::user::User;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use surrealdb::sql;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SpacetimeGeometry {
    pub geometry: geo::Geometry,
    pub timestamp: f32,
    pub duration: sql::Duration,
    pub title: String,
    pub text_body: String,
    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SpacetimePoint {
    pub geometry: geo::Point,
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
    pub geometry: GeoJson,
    pub timestamp: f32,
    pub duration: sql::Duration,
    pub title: String,
    pub text_body: String,
}

#[cfg(test)]
mod tests {
    use super::GeoJson;

    #[test]
    fn serialise_point() {
        // let point = Geometry::Point((0., 1.).into());

        // println!("{:?}", serde_json::to_string(&point));
    }
}
