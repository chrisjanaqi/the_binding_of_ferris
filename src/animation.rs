use crate::input::*;
use crate::player::Player;

use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// const FRAMERATE: u32 = 10;
pub const ZOOM: f32 = 6.0;

#[derive(Debug, Default)]
pub struct AnimTimer(pub Timer);

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone, Serialize, Deserialize)]
pub enum AnimState {
    Idle(AnimOrientation),
    Move(AnimOrientation),
    Attack(AnimOrientation),
    Hit(AnimOrientation),
    Die(AnimOrientation),
}

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone, Serialize, Deserialize)]
pub enum AnimOrientation {
    Up,
    Down,
    Side,
}

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone, Serialize, Deserialize)]
pub struct AnimData {
    pub start: u32,
    pub length: u32,
}

#[derive(Serialize, Deserialize)]
pub struct Animation {
    graph: HashMap<AnimState, AnimState>,
    data: HashMap<AnimState, AnimData>,
    current: AnimState,
    #[serde(skip)]
    index: u32,
    #[serde(skip)]
    paused: bool,
}

impl Animation {
    pub fn next_frame(&mut self) -> u32 {
        let index = self.index();
        if !self.paused {
            self.index += 1;
            if self.index == self.current().length {
                self.next_state();
            }
        }
        index
    }

    fn current(&self) -> AnimData {
        self.data[&self.current]
    }

    fn next_state(&mut self) {
        if let Some(&state) = self.graph.get(&self.current) {
            self.index = 0;
            self.current = state;
        }
    }

    pub fn index(&self) -> u32 {
        self.index + self.current().start
    }

    pub fn set_state(&mut self, state: AnimState) {
        if state != self.current && self.data.contains_key(&state) {
            self.index = 0;
            self.current = state;
            self.graph.entry(state).or_default();
        }
    }

    pub fn from_file(path: &str) -> Result<Self, ron::Error> {
        ron::from_str(&std::fs::read_to_string(path)?)
    }

    /// Creates a simple one state animation of given length
    pub fn from_length(length: u32) -> Self {
        Self {
            data: std::iter::once((
                Default::default(),
                AnimData {
                    length,
                    ..Default::default()
                },
            ))
            .collect(),
            ..Default::default()
        }
    }
}

pub struct IsaacAnimations;

impl IsaacAnimations {
    fn animation(
        time: Res<Time>,
        mut timer: ResMut<AnimTimer>,
        mut query: Query<(&mut TextureAtlasSprite, &mut Animation)>,
    ) {
        timer.0.tick(time.delta_seconds);
        for (mut sprite, mut animation) in query.iter_mut() {
            if timer.0.finished {
                sprite.index = animation.next_frame();
            }
        }
    }

    fn orientation(
        actions: ChangedRes<Actions<Action>>,
        mut query: Query<With<Player, &mut Transform>>,
    ) {
        for mut transform in query.iter_mut() {
            if let Some(Action::Shoot(direction)) = actions.get(Action::Shoot) {
                if direction.x() > f32::EPSILON {
                    *transform.scale.x_mut() = ZOOM;
                } else if direction.x() < -f32::EPSILON {
                    *transform.scale.x_mut() = -ZOOM;
                }
            } else if let Some(Action::Move(direction)) = actions.get(Action::Move) {
                if direction.x() > f32::EPSILON {
                    *transform.scale.x_mut() = ZOOM;
                } else if direction.x() < -f32::EPSILON {
                    *transform.scale.x_mut() = -ZOOM;
                }
            }
        }
    }
}

impl Plugin for IsaacAnimations {
    fn build(&self, app: &mut AppBuilder) {
        app.add_resource(AnimTimer(Timer::from_seconds(0.1, true)))
            .add_system(Self::animation.system())
            .add_system(Self::orientation.system());
    }
}

impl Default for AnimState {
    fn default() -> Self {
        Self::Idle(AnimOrientation::Down)
    }
}

impl Default for AnimOrientation {
    fn default() -> Self {
        Self::Down
    }
}

impl Default for Animation {
    fn default() -> Self {
        Self {
            graph: std::iter::once((Default::default(), Default::default())).collect(),
            data: std::iter::once((Default::default(), Default::default())).collect(),
            current: Default::default(),
            index: Default::default(),
            paused: false,
        }
    }
}

impl Default for AnimData {
    fn default() -> Self {
        Self {
            start: 0,
            length: 1,
        }
    }
}
