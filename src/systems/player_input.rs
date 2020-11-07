use crate::components::*;
use crate::model::inputs::*;
use crate::utils::*;

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
