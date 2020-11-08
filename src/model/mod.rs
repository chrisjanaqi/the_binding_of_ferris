pub mod behaviors;
pub mod camera;
pub mod inputs;
pub mod player;
pub mod tears;

pub use camera::*;
pub use inputs::*;
pub use player::*;
pub use tears::*;

pub struct DeltaTime(pub f32);

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Cooldown {
    duration: f32,
    current: f32,
    available: bool,
}

impl Cooldown {
    pub fn new(duration: f32) -> Self {
        Self {
            duration,
            current: duration,
            available: true,
        }
    }

    pub fn reset(&mut self) {
        self.current = self.duration;
        self.available = true;
    }

    pub fn update(&mut self, dt: f32) {
        if !self.available {
            self.current -= dt;
            if self.current <= 0. {
                self.reset();
            }
        }
    }

    pub fn start(&mut self) {
        if self.available() {
            self.available = false;
        }
    }

    pub fn available(&self) -> bool {
        self.available
    }

    // pub fn change_duration(&mut self, duration: f32) {
    //     self.duration = duration;
    // }
}
