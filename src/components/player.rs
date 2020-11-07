use crate::utils::*;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Default)]
pub struct TagPlayer;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PlayerStats {
    pub health: f32,
    pub attack_damage: f32,
    pub attack_cooldown: f32,
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
            attack_cooldown: 5.0,
            tear_lifetime: 3.0,
            tear_speed: 500.0,
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
