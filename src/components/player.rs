use crate::model::Cooldown;
use crate::utils::*;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Default)]
pub struct TagPlayer;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PlayerStats {
    pub health: f32,
    pub attack_damage: f32,
    pub attack_cooldown: Cooldown,
    pub tear_lifetime: f32,
    pub tear_speed: f32,
    pub luck: f32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PlayerAction {
    pub shoot: Vector,
}

impl Default for PlayerStats {
    fn default() -> Self {
        Self {
            health: 10.0,
            attack_damage: 10.0,
            attack_cooldown: Cooldown::new(0.5),
            tear_lifetime: 1.5,
            tear_speed: 0.5,
            luck: 0.0,
        }
    }
}

impl Default for PlayerAction {
    fn default() -> Self {
        Self {
            shoot: Vector::zeros(),
        }
    }
}
