use serde::{Deserialize, Serialize};

use crate::color::Color;

#[derive(Serialize, Deserialize)]
pub struct Fill {
    color: Color,
    opacity: u8,
}

#[derive(Serialize, Deserialize)]
pub enum Dash {
    Other(String),
}
#[derive(Serialize, Deserialize)]
pub struct Stroke {
    color: Color,
    opacity: u8,
    weight: u8,
    dash: Dash,
}

#[derive(Serialize, Deserialize)]
pub enum Filter {}

#[derive(Serialize, Deserialize)]
pub struct Style {
    fill: Option<Fill>,
    stroke: Option<Stroke>,
    filter: Option<Filter>,
}

impl Style {
    pub fn fill(mut self, fill: Fill) -> Style {
        self.fill = Some(fill);
        self
    }
    pub fn stroke(mut self, stroke: Stroke) -> Style {
        self.stroke = Some(stroke);
        self
    }
    pub fn filter(mut self, filter: Filter) -> Style {
        self.filter = Some(filter);
        self
    }
}
