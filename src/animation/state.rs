use serde::{Deserialize, Serialize};

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone, Serialize, Deserialize)]
pub enum AnimState {
    Idle(AnimOrientation),
    Move(AnimOrientation),
    Attack(AnimOrientation),
    Hit(AnimOrientation),
    Die(AnimOrientation),
}

impl Default for AnimState {
    fn default() -> Self {
        Self::Idle(Default::default())
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone, Serialize, Deserialize)]
pub enum AnimOrientation {
    Up,
    Down,
    Side,
}

impl Default for AnimOrientation {
    fn default() -> Self {
        Self::Down
    }
}
