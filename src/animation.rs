use crate::input::*;
use crate::player::Player;

use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// const FRAMERATE: u32 = 10;
pub const ZOOM: f32 = 6.0;

#[derive(Debug, Default)]
pub struct AnimTimer(pub Timer);

impl AnimTimer {
    fn reset(&mut self) {
        // self.0.reset();
        self.0.finished = true;
        self.0.just_finished = true;
    }
}

pub struct PlayerAnimEvent {
    // pub priority: i32,
    pub state: AnimState,
}

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
    pub priority: i32,
}

#[derive(Serialize, Deserialize)]
pub struct Animation {
    graph: HashMap<AnimState, AnimState>,
    data: HashMap<AnimState, AnimData>,
    current: AnimState,
    #[serde(skip)]
    next: Option<AnimState>,
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
        if let Some(state) = self.next {
            // println!("Next state is {:?}", state);
            self.index = 0;
            self.current = state;
            self.next = None;
        } else if let Some(&state) = self.graph.get(&self.current) {
            // println!("Next state is {:?}", state);
            self.index = 0;
            self.current = state;
        }
    }

    pub fn index(&self) -> u32 {
        self.index + self.current().start
    }

    pub fn set_state(&mut self, state: AnimState) {
        if state != self.current && self.data.contains_key(&state) {
            let state_priority = self.data[&state].priority;
            let current_priority = self.data[&self.current].priority;
            if state_priority >= current_priority {
                println!("Setting state to {:?}", state);
                if state_priority > current_priority {
                    self.next = Some(self.current);
                }
                self.index = 0;
                self.current = state;
                self.graph.entry(state).or_default();
            } else {
                println!("Setting next state to {:?}", state);
                self.next = Some(state);
            }
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
    fn animation_update(
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

    fn player_animation(
        mut reader: Local<EventReader<PlayerAnimEvent>>,
        actions: ChangedRes<Actions<Action>>,
        anim_events: Res<Events<PlayerAnimEvent>>,
        mut timer: ResMut<AnimTimer>,
        mut query: Query<With<Player, (&mut Transform, &mut Animation)>>,
    ) {
        for (mut transform, mut animation) in query.iter_mut() {
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
            for player_anim in reader.iter(&anim_events) {
                animation.set_state(player_anim.state);
                timer.reset();
            }
        }
    }
}

impl Plugin for IsaacAnimations {
    fn build(&self, app: &mut AppBuilder) {
        app.add_resource(AnimTimer(Timer::from_seconds(0.1, true)))
            .add_event::<PlayerAnimEvent>()
            .add_system(Self::player_animation.system())
            .add_system(Self::animation_update.system());
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
            next: Default::default(),
            index: Default::default(),
            paused: false,
        }
    }
}

impl Default for AnimData {
    fn default() -> Self {
        Self {
            priority: 0,
            start: 0,
            length: 1,
        }
    }
}
