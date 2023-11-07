use models::{
    body::Body,
    user::{CreateUser, User},
};

use axum::{extract::State, response::Json};

use crate::{error::ApiError, AppState};

pub async fn get_users(State(state): State<AppState>) -> Result<Json<Vec<Body<User>>>, ApiError> {
    let resp = state.db.get_users().await?;

    Ok(Json(resp))
}
