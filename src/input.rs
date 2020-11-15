use bevy::prelude::*;
use log::debug;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
struct KeyBindings(HashMap<KeyCode, Action>);

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone, Serialize, Deserialize)]
pub enum Action {
    Move(Direction),
    Shoot(Direction),
    // Item,
    // Bomb,
    // Card,
    // Drop,
}

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone, Serialize, Deserialize)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl KeyBindings {
    fn get(&self, key: &KeyCode) -> Option<Action> {
        if let Some(&action) = self.0.get(key) {
            Some(action)
        } else {
            None
        }
    }

    fn wasd() -> Self {
        use self::Direction::*;
        use Action::*;
        Self(
            vec![
                (KeyCode::W, Move(Up)),
                (KeyCode::A, Move(Left)),
                (KeyCode::S, Move(Down)),
                (KeyCode::D, Move(Right)),
                (KeyCode::Up, Shoot(Up)),
                (KeyCode::Left, Shoot(Left)),
                (KeyCode::Down, Shoot(Down)),
                (KeyCode::Right, Shoot(Right)),
            ]
            .into_iter()
            .collect(),
        )
    }

    fn from_file(path: &str) -> Result<Self, ron::Error> {
        ron::from_str(&std::fs::read_to_string(path)?)
    }
}

impl Default for KeyBindings {
    fn default() -> Self {
        Self::wasd()
    }
}

pub struct IsaacInputs;

impl IsaacInputs {
    // const STAGE: &'static str = "isaac_input";

    fn keyboard(
        mut actions: ResMut<Input<Action>>,
        keys: Res<Input<KeyCode>>,
        bindings: Res<KeyBindings>,
    ) {
        actions.update();
        keys.get_just_pressed()
            .filter_map(|key| bindings.get(key))
            .for_each(|action| {
                debug!("Pressed {:?}", action);
                actions.press(action);
            });

        keys.get_just_released()
            .filter_map(|key| bindings.get(key))
            .for_each(|action| {
                debug!("Released {:?}", action);
                actions.release(action);
            });
    }
}

impl Plugin for IsaacInputs {
    fn build(&self, app: &mut AppBuilder) {
        let bindings = KeyBindings::from_file("key_bindings.ron").unwrap_or_default();
        app.add_resource(bindings)
            .init_resource::<Input<Action>>()
            //.add_stage(IsaacInputs::STAGE)
            .add_system(IsaacInputs::keyboard.system());
    }
}
