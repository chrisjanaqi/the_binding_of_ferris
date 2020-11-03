use ggez::nalgebra as na;

type Point = na::Point2<f32>;
type Vector = na::Vector2<f32>;

/// Structure representing an angle in radian
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct Radian(f32);

impl From<f32> for Radian {
    fn from(val: f32) -> Self {
        Self(val.rem_euclid(2.0 * std::f32::consts::PI))
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Translation(Point);

impl Default for Translation {
    fn default() -> Self {
        Self(Point::new(0.0, 0.0))
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Velocity(Vector);

impl Default for Velocity {
    fn default() -> Self {
        Self(Vector::new(0.0, 0.0))
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct Rotation(Radian);

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct AngularVelocity(Radian);

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct Size(f32);

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct Lifetime(f32);
