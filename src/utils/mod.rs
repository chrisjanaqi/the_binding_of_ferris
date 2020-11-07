use ggez::nalgebra as na;

pub type Point = na::Point2<f32>;
pub type Vector = na::Vector2<f32>;

/// Structure representing an angle in radian
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct Radian(pub f32);

impl From<f32> for Radian {
    fn from(val: f32) -> Self {
        Self(val.rem_euclid(2.0 * std::f32::consts::PI))
    }
}
impl From<Radian> for f32 {
    fn from(val: Radian) -> Self {
        val.0
    }
}

/// Clamps the norm of a vector v under max.
pub fn clamp_norm(v: Vector, max: f32) -> Vector {
    assert!(max > 0.0);
    let norm = v.norm();

    if norm > max {
        v * max / norm
    } else {
        v
    }
}

pub fn input_to_vector(up: bool, down: bool, left: bool, right: bool) -> Vector {
    let x = (right as i32 - left as i32) as f32;
    let y = (down as i32 - up as i32) as f32;
    Vector::new(x, y)
}