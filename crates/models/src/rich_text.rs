use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum Block {
    Paragraph(String),
}

pub type RichText = Vec<Block>;
