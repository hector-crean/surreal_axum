use serde::{Deserialize, Serialize};
use surrealdb::sql;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Body<Payload> {
    pub id: sql::Thing,
    #[serde(flatten)]
    pub payload: Payload,
}
