use crate::record::Record;

use super::session_token::SessionToken;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use strum::AsRefStr;
use surrealdb::sql::{Datetime, Index, Thing};
use uuid::Uuid;

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
    pub role: Role,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct User {
    pub name: String,
    pub username: String,
    pub password: String,
    pub role: Role,
}

pub type UserRecord = Record<User>;
