mod physics;
mod player;

pub use physics::*;
pub use player::*;

use crate::utils::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WorldToScreen {
    pub t: Point,
    pub r: Radian,
    pub s: f32,
}

impl Default for WorldToScreen {
    fn default() -> Self {
        Self {
            t: Point::new(0.0, 0.0),
            r: Radian(0.0),
            s: 1.0,
        }
    }
}
