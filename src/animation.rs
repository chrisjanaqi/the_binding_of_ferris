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

#[derive(Debug, Default, Eq, PartialEq, Hash, Copy, Clone, Serialize, Deserialize)]
pub struct AnimConfig {
    pub start: u32,
    pub length: u32,
    pub end: AnimState,
}

#[derive(Serialize, Deserialize)]
pub struct Animation {
    graph: HashMap<AnimState, AnimConfig>,
    state: AnimState,
    #[serde(skip)]
    index: u32,
}

impl Animation {
    pub fn next_frame(&mut self) -> u32 {
        self.index += 1;
        if self.index == self.current().length {
            self.next_state();
        }
        self.index()
    }

    fn current(&self) -> AnimConfig {
        self.graph[&self.state]
    }

    fn next_state(&mut self) {
        self.index = 0;
        self.state = self.current().end;
    }

    pub fn index(&self) -> u32 {
        self.index + self.current().start
    }

    pub fn set_state(&mut self, state: AnimState) {
        if state != self.state {
            self.index = 0;
            self.state = state;
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
}

impl Plugin for IsaacAnimations {
    fn build(&self, app: &mut AppBuilder) {
        app.add_resource(AnimTimer(Timer::from_seconds(0.1, true)))
            .add_system(Self::animation.system());
    }
}

impl Default for AnimState {
    fn default() -> Self {
        Self::Idle(AnimOrientation::Down)
    }
}

impl Default for Animation {
    fn default() -> Self {
        Self {
            graph: std::iter::once((Default::default(), Default::default())).collect(),
            state: Default::default(),
            index: Default::default(),
        }
    }
}
