use models::{
    body::Body,
    user::{CreateUser, User},
};

use axum::{extract::State, response::Json};

use rand_core::OsRng;
use uuid::Uuid;

use crate::{error::ApiError, AppState};

pub async fn create_user(
    State(state): State<AppState>,
    payload: Json<CreateUser>,
) -> Result<Json<Vec<Body<User>>>, ApiError> {
    let resp = state.db.create_user(payload).await?;

    Ok(Json(resp))
}
