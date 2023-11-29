use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use surrealdb::sql;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Record<Data> {
    pub id: sql::Thing,
    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
    #[serde(flatten)]
    pub data: Data,
}
