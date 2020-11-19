use bevy::prelude::*;
use log::debug;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::mem::{discriminant, Discriminant};

#[derive(Serialize, Deserialize)]
struct KeyBindings(HashMap<KeyCode, Action>);

pub struct Actions<T> {
    /// Currently active player actions
    active: HashMap<Discriminant<T>, T>,
    /// New actions of the player this frame
    new: HashMap<Discriminant<T>, T>,
    /// Action finished this frame
    finished: HashMap<Discriminant<T>, T>,
}

impl<T> Actions<T>
where
    T: Copy,
{
    pub fn update(&mut self) {
        self.new.clear();
        self.finished.clear();
    }

    pub fn start(&mut self, action: T) {
        let key = discriminant(&action);
        if !self.active.contains_key(&key) {
            self.new.insert(key, action);
        }

        self.active.insert(key, action);
    }

    pub fn stop(&mut self, action: T) {
        let key = discriminant(&action);
        if let Some(action) = self.active.remove(&key) {
            self.finished.insert(key, action);
        }
    }

    pub fn get(&self, action: T) -> Option<&T> {
        self.active.get(&discriminant(&action))
    }

    // pub fn just_started(&self, action: T) -> Option<&T> {
    //     self.new.get(&discriminant(&action))
    // }

    // pub fn just_finished(&self, action: T) -> Option<&T> {
    //     self.finished.get(&discriminant(&action))
    // }

    // pub fn active_actions(&self) -> impl ExactSizeIterator<Item = &T> {
    //     self.active.values()
    // }

    // pub fn new_actions(&self) -> impl ExactSizeIterator<Item = &T> {
    //     self.new.values()
    // }

    // pub fn finished_actions(&self) -> impl ExactSizeIterator<Item = &T> {
    //     self.finished.values()
    // }
}

impl<T> Default for Actions<T> {
    fn default() -> Self {
        Self {
            active: Default::default(),
            new: Default::default(),
            finished: Default::default(),
        }
    }
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum Action {
    Move(Vec2),
    Shoot(Vec2),
    Item,
    Bomb,
    Card,
    // Drop,
}

impl Action {
    fn is_state(&self) -> bool {
        matches!(self, Self::Move(_) | Self::Shoot(_))
    }
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
        use Action::*;
        Self(
            vec![
                (KeyCode::W, Move(Vec2::new(0.0, 1.0))),
                (KeyCode::A, Move(Vec2::new(-1.0, 0.0))),
                (KeyCode::S, Move(Vec2::new(0.0, -1.0))),
                (KeyCode::D, Move(Vec2::new(1.0, 0.0))),
                (KeyCode::Up, Shoot(Vec2::new(0.0, 1.0))),
                (KeyCode::Left, Shoot(Vec2::new(-1.0, 0.0))),
                (KeyCode::Down, Shoot(Vec2::new(0.0, -1.0))),
                (KeyCode::Right, Shoot(Vec2::new(1.0, 0.0))),
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
        mut actions: ResMut<Actions<Action>>,
        keys: Res<Input<KeyCode>>,
        bindings: Res<KeyBindings>,
    ) {
        actions.update();

        keys.get_just_pressed()
            .filter_map(|key| bindings.get(key))
            .filter(|action| !action.is_state())
            .for_each(|action| {
                debug!("Pressed {:?}", action);
                actions.start(action);
            });

        // updating the actions that requires several inputs to be calculated
        let mut direction = Vec2::default();
        let mut shoot_direction = Vec2::default();

        keys.get_pressed()
            .filter_map(|key| bindings.get(key))
            .filter(|action| action.is_state())
            .for_each(|action| match action {
                Action::Move(dir) => direction += dir,
                Action::Shoot(dir) => shoot_direction += dir,
                _ => (),
            });

        if direction.length_squared() > f32::EPSILON {
            actions.start(Action::Move(direction.normalize()));
        } else {
            actions.stop(Action::Move(Default::default()));
        }

        if shoot_direction.length_squared() > f32::EPSILON {
            actions.start(Action::Shoot(shoot_direction.normalize()));
        } else {
            actions.stop(Action::Shoot(Default::default()));
        }

        keys.get_just_released()
            .filter_map(|key| bindings.get(key))
            .filter(|action| !action.is_state())
            .for_each(|action| {
                debug!("Released {:?}", action);
                actions.stop(action);
            });
    }
}

impl Plugin for IsaacInputs {
    fn build(&self, app: &mut AppBuilder) {
        let bindings = KeyBindings::from_file("key_bindings.ron").unwrap_or_default();
        app.add_resource(bindings)
            .init_resource::<Actions<Action>>()
            //.add_stage(IsaacInputs::STAGE)
            .add_system(IsaacInputs::keyboard.system());
    }
}
