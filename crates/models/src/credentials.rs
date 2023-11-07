use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Credentials<'a> {
    pub name: &'a str,
    pub username: &'a str,
    pub password: &'a str,
}
