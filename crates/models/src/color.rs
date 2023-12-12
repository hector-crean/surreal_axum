use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum Color {
    Rgba { r: u8, g: u8, b: u8, a: u8 },
    Hsl { h: u8, s: u8, l: u8, a: u8 },
}

impl Default for Color {
    fn default() -> Self {
        Self::Rgba {
            r: 0,
            g: 0,
            b: 0,
            a: 1,
        }
    }
}
