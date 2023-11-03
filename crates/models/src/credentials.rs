use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Credentials<'a> {
    email: &'a str,
    pass: &'a str,
}
