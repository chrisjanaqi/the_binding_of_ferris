// use crate::behaviors::{PlayerBehavior, TearBehavior};
use crate::utils::*;

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

/*
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct Lifetime(pub f32);

#[derive(Default)]
pub struct TearBehaviors(pub Vec<Box<dyn TearBehavior>>);

#[derive(Default)]
pub struct PlayerBehaviors(pub Vec<Box<dyn PlayerBehavior>>);


pub struct TearStats {
    pub lifetime: f32,
    pub damage: f32,
}

// Bunch of tags that affects the tears
#[derive(Clone, Copy, Debug, Eq, PartialEq, Default)]
pub struct TagSpectral;
#[derive(Clone, Copy, Debug, Eq, PartialEq, Default)]
pub struct TagPiercing;
#[derive(Clone, Copy, Debug, Eq, PartialEq, Default)]
pub struct TagExplosive;
#[derive(Clone, Copy, Debug, Eq, PartialEq, Default)]
pub struct TagBreaking;
// */

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct PlayerStats {
    pub health: f32,
    pub movement_speed: f32,
    pub attack_speed: f32,
    pub attack_damage: f32,
    pub tear_lifetime: f32,
    pub tear_speed: f32,
    pub luck: f32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PlayerAction {
    pub shoot: Vector,
}

///
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Movement {
    pub direction: Vector,
    pub speed: f32,
    pub acceleration: f32,
    pub damping: f32,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Default)]
pub struct TagPlayer;

/*
 *  Traits Implementations
 */
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
            speed: 400.0,
            acceleration: 400.0,
            damping: 400.0,
        }
    }
}
