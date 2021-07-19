mod cache;
mod hint;
mod proxy;
mod scale;
mod var;

use super::internal;

pub use proxy::GlyfProxy;
pub use scale::{Scaler, ScalerState};

#[derive(Copy, Clone, Default, Debug)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}
