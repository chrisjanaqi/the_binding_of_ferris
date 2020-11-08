use crate::components::*;
use crate::model::inputs::*;
use crate::utils::*;

use legion::*;

#[system(for_each)]
pub fn player_input(
    movement: &mut Movement,
    action: &mut PlayerAction,
    rotation: &mut Rotation,
    _: &TagPlayer,
    #[resource] inputs: &Inputs,
    #[resource] bindings: &KeyBindings,
) {
    let kb = &inputs.keyboard;
    let directions: Vec<_> = vec![bindings.up, bindings.down, bindings.left, bindings.right]
        .into_iter()
        .map(|key| kb.pressed(key))
        .collect();
    let moving = directions.iter().any(|b| *b);
    movement.direction = clamp_norm(input_to_vector(&directions), 1.0);

    let shoot: Vec<_> = vec![
        bindings.shoot_up,
        bindings.shoot_down,
        bindings.shoot_left,
        bindings.shoot_right,
    ]
    .into_iter()
    .map(|key| kb.pressed(key))
    .collect();
    let shooting = shoot.iter().any(|b| *b);
    action.shoot = normalize(input_to_vector(&shoot));

    if shooting {
        rotation.0 = angle(action.shoot);
    } else if moving {
        rotation.0 = angle(movement.direction);
    }
}
