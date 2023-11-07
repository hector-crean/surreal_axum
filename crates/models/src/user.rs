use super::session_token::SessionToken;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use strum::AsRefStr;
use surrealdb::sql::{Datetime, Index, Thing};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct User {
    pub name: String,
    pub username: String,
    // #[serde(skip_serializing)]
    pub password: String,
    // pub role: Role,
    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize, AsRefStr)]
pub enum Role {
    User,
    Superuser,
    Admin,
    Moderator,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateUser {
    pub name: String,
    pub username: String,
    pub password: String,
    // pub role: Role,
}
