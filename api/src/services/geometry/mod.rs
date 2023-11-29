use crate::error::ApiError;
use crate::AppState;
use axum::extract::Path;
use axum::extract::State;
use axum::Json;
use geo_types::{
    CoordNum, GeometryCollection, Line, LineString, MultiLineString, MultiPoint, MultiPolygon,
    Point, Polygon, Rect, Triangle,
};
use geojson::Feature;
use geojson::{de::deserialize_geometry, ser::serialize_geometry, FeatureReader, FeatureWriter};
use models::user::{CreateUser, User};
use serde::{Deserialize, Serialize};

#[derive(Eq, PartialEq, Clone, Debug, Hash, Serialize, Deserialize)]
#[serde(tag = "type", content = "coordinates")]
pub enum Geometry2D<T: CoordNum = f64> {
    Point(Point<T>),
    Line(Line<T>),
    LineString(LineString<T>),
    Polygon(Polygon<T>),
    MultiPoint(MultiPoint<T>),
    MultiLineString(MultiLineString<T>),
    MultiPolygon(MultiPolygon<T>),
    GeometryCollection(GeometryCollection<T>),
    Rect(Rect<T>),
    Triangle(Triangle<T>),
}

pub struct LineStyle {
    pub opacity: Option<f64>,
}

pub enum Geometry2DStyle {}

#[derive(Serialize, Deserialize)]
pub struct Annotation {
    #[serde(
        serialize_with = "serialize_geometry",
        deserialize_with = "deserialize_geometry"
    )]
    pub geometry: geo_types::Geometry,
    pub style: Geometry2DStyle,
}

#[cfg(test)]
mod tests {
    use geo_types::{coord, point};

    use super::*;

    #[test]
    fn serde_test() {
        let an = Annotation {
            geometry: point! {x: 1.0, y: 2.0}.into(),
        };
        println!("{:?}", &serde_json::to_string(&an));
    }
}

const GEOMETRY_TABLE: &str = "geometry";

pub async fn create(
    State(AppState { db, .. }): State<AppState>,
    Json(data): Json<Geometry2D>,
) -> Result<Json<Vec<Geometry2D>>, ApiError> {
    let geometry = db.client.create(GEOMETRY_TABLE).content(data).await?;
    Ok(Json(geometry))
}

pub async fn read(
    State(AppState { db, .. }): State<AppState>,
    id: Path<String>,
) -> Result<Json<Option<Geometry2D>>, ApiError> {
    let geometry = db.client.select((GEOMETRY_TABLE, &*id)).await?;
    Ok(Json(geometry))
}

pub async fn list(
    State(AppState { db, .. }): State<AppState>,
) -> Result<Json<Vec<Geometry2D>>, ApiError> {
    let geometries = db.client.select(GEOMETRY_TABLE).await?;
    Ok(Json(geometries))
}
