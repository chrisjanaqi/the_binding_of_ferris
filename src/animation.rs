use crate::Velocity;

use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// const FRAMERATE: u32 = 10;

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
        if !self.paused {
            self.index += 1;
            if self.index == self.current().length {
                self.next_state();
            }
        }
        self.index()
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
        if state != self.current {
            self.index = 0;
            self.current = state;
        }
    }

    pub fn from_file(path: &str) -> Result<Self, ron::Error> {
        ron::from_str(&std::fs::read_to_string(path)?)
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

    fn orientation(mut query: Query<(&mut Transform, Changed<Velocity>)>) {
        for (mut transform, velocity) in query.iter_mut() {
            let scale = transform.scale.x().abs();
            if velocity.0.x() > f32::EPSILON {
                *transform.scale.x_mut() = scale;
            } else if velocity.0.x() < -f32::EPSILON {
                *transform.scale.x_mut() = -scale;
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
