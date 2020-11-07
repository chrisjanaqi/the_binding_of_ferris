use crate::utils::*;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Camera {
    pub position: Point,
    pub rotation: Radian,
    pub zoom: f32,
}

impl Camera {
    pub const WIDTH: f32 = 1920.0;
    pub const HEIGHT: f32 = 1080.0;
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            position: Point::new(0.0, 0.0),
            rotation: Radian(0.0),
            zoom: 1.0,
        }
    }
}
