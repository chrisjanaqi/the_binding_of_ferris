use crate::components::*;
use crate::inputs::*;
use crate::utils::*;
use crate::DeltaTime;

use legion::*;

#[system(for_each)]
pub fn player_input(
    player_movement: &mut Movement,
    _: &TagPlayer,
    #[resource] inputs: &Inputs,
    #[resource] bindings: &KeyBindings,
) {
    let kb = &inputs.keyboard;
    let movement: Vec<_> = vec![bindings.up, bindings.down, bindings.left, bindings.right]
        .into_iter()
        .map(|key| kb.pressed(key))
        .collect();

    player_movement.direction = clamp_norm(
        input_to_vector(movement[0], movement[1], movement[2], movement[3]),
        1.0,
    );
}

#[system(for_each)]
pub fn moving(movement: &Movement, velocity: &mut Velocity, #[resource] delta: &DeltaTime) {
    let DeltaTime(dt) = *delta;
    let target = movement.direction * movement.speed;
    let current = velocity.0;
    let error = target - current;

    let norm = error.norm();
    if norm > std::f32::EPSILON {
        let coeff = if target.norm() <= std::f32::EPSILON {
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
