use crate::utils::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Translation(pub Point);

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Velocity(pub Vector);

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct Rotation(pub Radian);

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct AngularVelocity(pub Radian);

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Size(pub f32);

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Movement {
    pub direction: Vector,
    pub speed: f32,
    pub acceleration: f32,
    pub damping: f32,
}

impl Default for Translation {
    fn default() -> Self {
        Self(Point::new(0.0, 0.0))
    }
}

impl Default for Velocity {
    fn default() -> Self {
        Self(Vector::new(0.0, 0.0))
    }
}

impl Default for Size {
    fn default() -> Self {
        Self(1.0)
    }
}

impl Default for Movement {
    fn default() -> Self {
        Self {
            direction: Vector::zeros(),
            speed: 0.5,
            acceleration: 5.0,
            damping: 1.5,
        }
    }
}
