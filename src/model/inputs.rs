use ggez::event::{KeyCode, MouseButton};
use ggez::input::*;
use std::collections::HashSet;

#[derive(Default)]
pub struct Inputs {
    pub keyboard: Keyboard,
    pub mouse: Mouse,
}

impl Inputs {
    /// Update the inputs with a new ggez context.
    ///
    /// This function must be called every frame.
    pub fn update(&mut self, ctx: &ggez::Context) {
        self.keyboard.update(ctx);
        self.mouse.update(ctx);
    }
}

pub struct Mouse {
    pub buttons: HashSet<MouseButton>,
    pub position: (f32, f32),
    pub delta: (f32, f32),
    pub wheel: (f32, f32),
}

impl Mouse {
    fn update(&mut self, ctx: &ggez::Context) {
        self.delta = {
            let d = mouse::delta(ctx);
            (d.x, d.y)
        };
        self.position = {
            let p = mouse::position(ctx);
            (p.x, p.y)
        };
        // TODO: Mouse wheel, Mouse buttons
    }
}

pub struct Keyboard {
    keys: HashSet<KeyCode>,
    modifiers: keyboard::KeyMods,
    hold: bool,
}

pub struct KeyBindings {
    pub up: KeyCode,
    pub down: KeyCode,
    pub left: KeyCode,
    pub right: KeyCode,
    pub shoot_up: KeyCode,
    pub shoot_down: KeyCode,
    pub shoot_left: KeyCode,
    pub shoot_right: KeyCode,
}

impl Keyboard {
    fn update(&mut self, ctx: &ggez::Context) {
        self.keys.clear();
        self.keys.extend(keyboard::pressed_keys(ctx));
        self.modifiers = keyboard::active_mods(ctx);
        self.hold = keyboard::is_key_repeated(ctx);
    }

    pub fn pressed(&self, key: KeyCode) -> bool {
        self.keys.contains(&key)
    }

    /*
    pub fn held(&self, key: KeyCode) -> bool {
        self.pressed(key) && self.hold
    }

    /// Checks if the given modifiers are active
    pub fn modifier(&self, key_mods: keyboard::KeyMods) -> bool {
        (self.modifiers & key_mods) == key_mods
    }
    */
}

impl Default for Mouse {
    fn default() -> Self {
        Self {
            buttons: HashSet::with_capacity(3),
            position: Default::default(),
            delta: Default::default(),
            wheel: Default::default(),
        }
    }
}

impl Default for Keyboard {
    fn default() -> Self {
        Self {
            keys: HashSet::with_capacity(10),
            modifiers: Default::default(),
            hold: false,
        }
    }
}

impl Default for KeyBindings {
    fn default() -> Self {
        Self {
            up: KeyCode::W,
            down: KeyCode::S,
            left: KeyCode::A,
            right: KeyCode::D,
            shoot_up: KeyCode::Up,
            shoot_down: KeyCode::Down,
            shoot_left: KeyCode::Left,
            shoot_right: KeyCode::Right,
        }
    }
}
