use models::spacetime_geometry::SpacetimeGeometry;

use axum::{extract::State, response::Json};

use crate::{error::ApiError, AppState};

pub async fn get_spacetime_geometries(
    State(state): State<AppState>,
) -> Result<Json<Vec<SpacetimeGeometry>>, ApiError> {
    let resp = state.db.get_spacetime_geometries().await?;

    Ok(Json(resp))
}
