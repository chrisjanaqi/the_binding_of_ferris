use crate::components::*;
use crate::model::*;

use legion::*;

#[system(for_each)]
pub fn moving(
    movement: &Movement,
    velocity: &mut Velocity,
    #[resource] delta: &DeltaTime,
) {
    let DeltaTime(dt) = *delta;
    let target = movement.direction * movement.speed;
    let current = velocity.0;
    let error = target - current;
    let moving = target.norm() > std::f32::EPSILON;

    let norm = error.norm();
    if norm > std::f32::EPSILON {
        let coeff = if !moving {
            movement.damping
        } else {
            movement.acceleration
        };

        velocity.0 = if norm <= coeff * dt {
            target
        } else {
            current + coeff * dt / norm * error
        };
    }
}

#[system(for_each)]
pub fn linear_simulation(
    translation: &mut Translation,
    velocity: &Velocity,
    #[resource] delta: &DeltaTime,
) {
    let DeltaTime(dt) = *delta;
    translation.0 += velocity.0 * dt;
}

#[system(for_each)]
pub fn angular_simulation(
    rotation: &mut Rotation,
    velocity: &AngularVelocity,
    #[resource] delta: &DeltaTime,
) {
    let DeltaTime(dt) = *delta;
    rotation.0.0 += velocity.0.0 * dt;
}