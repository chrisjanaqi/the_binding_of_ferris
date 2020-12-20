use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::AnimState;

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone, Serialize, Deserialize)]
pub struct AnimElement {
    pub start: u32,
    pub length: u32,
    pub priority: i32,
}

impl Default for AnimElement {
    fn default() -> Self {
        Self {
            priority: 0,
            start: 0,
            length: 1,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Animation {
    graph: HashMap<AnimState, AnimState>,
    data: HashMap<AnimState, AnimElement>,
    current: AnimState,
    #[serde(skip)]
    next: Option<AnimState>,
    #[serde(skip)]
    index: u32,
    #[serde(skip)]
    paused: bool,
}

impl Animation {
    fn current(&self) -> AnimElement {
        self.data[&self.current]
    }

    fn next_state(&mut self) {
        if let Some(state) = self.next {
            self.reset(state);
            self.next = None;
        } else if let Some(&state) = self.graph.get(&self.current) {
            self.reset(state);
        }
    }

    fn is_valid_state(&self, state: AnimState) -> bool {
        self.data.contains_key(&state)
    }

    fn get_priority(&self, state: AnimState) -> i32 {
        self.data[&state].priority
    }

    fn index(&self) -> u32 {
        self.index + self.current().start
    }

    fn reset(&mut self, state: AnimState) {
        self.index = 0;
        self.current = state;
        self.graph.entry(state).or_default();
    }

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

    pub fn set_state(&mut self, state: AnimState) {
        if state != self.current && self.is_valid_state(state) {
            let state_priority = self.get_priority(state);
            let current_priority = self.get_priority(self.current);
            if state_priority >= current_priority {
                if state_priority > current_priority {
                    self.next = Some(self.current);
                }
                self.reset(state);
            } else {
                self.next = Some(state);
            }
        }
    }

    /// Creates an animation of given length with a unique looping state
    pub fn from_length(length: u32) -> Self {
        Self {
            data: std::iter::once((
                Default::default(),
                AnimElement {
                    length,
                    ..Default::default()
                },
            ))
            .collect(),
            ..Default::default()
        }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_animation_from_length() {
        let mut animation = Animation::from_length(2);
        assert_eq!(0, animation.next_frame());
        assert_eq!(1, animation.next_frame());
        assert_eq!(0, animation.next_frame());
    }
}
