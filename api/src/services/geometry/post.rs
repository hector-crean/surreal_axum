use crate::{error::ApiError, AppState};
use axum::{extract::State, response::Json};
use models::{
    body::Body,
    spacetime_geometry::{CreateSpacetimeGeometry, SpacetimeGeometry},
};

pub async fn create_spacetime_geometry(
    State(state): State<AppState>,
    payload: Json<CreateSpacetimeGeometry>,
) -> Result<Json<Option<Body<SpacetimeGeometry>>>, ApiError> {
    let resp = state.db.create_spacetime_geometry(payload).await?;

    Ok(Json(resp))
}
