// use geo::Geometry;
// use geo::{
//     CoordNum, GeometryCollection, Line, LineString, MultiLineString, MultiPoint, MultiPolygon,
//     Point, Polygon, Rect, Triangle,
// };
use geojson::Geometry;
use serde::{Deserialize, Serialize};

use geojson::{LineStringType, PointType, PolygonType};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type", content = "coordinates")]
pub enum GeoJson {
    Point(PointType),
    MultiPoint(Vec<PointType>),
    LineString(LineStringType),
    MultiLineString(Vec<LineStringType>),
    Polygon(PolygonType),
    MultiPolygon(Vec<PolygonType>),
    // GeometryCollection(Vec<Geometry>),
}

// #[derive(Eq, PartialEq, Clone, Debug, Hash, Serialize, Deserialize)]
// #[serde(tag = "type", content = "coordinates")]
// pub enum Geometry<T: CoordNum = f64> {
//     Point(Point<T>),
//     Line(Line<T>),
//     LineString(LineString<T>),
//     Polygon(Polygon<T>),
//     MultiPoint(MultiPoint<T>),
//     MultiLineString(MultiLineString<T>),
//     MultiPolygon(MultiPolygon<T>),
// }
