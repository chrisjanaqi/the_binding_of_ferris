mod animation_graph;
mod state;
pub use animation_graph::Animation;
pub use state::{AnimOrientation, AnimState};

use crate::input::*;
use crate::player::Player;

use bevy::prelude::*;

pub const ZOOM: f32 = 6.0;

#[derive(Bundle)]
pub struct AnimationBundle {
    pub animation: Animation,
    pub anim_timer: AnimTimer,
}

#[derive(Debug)]
pub struct AnimTimer(pub Timer);

impl AnimTimer {
    pub fn new(fps: f32) -> Self {
        let duration = 1.0 / fps;
        let mut timer = Timer::from_seconds(duration, true);
        timer.tick(duration);
        Self(timer)
    }

    pub fn reset(&mut self) {
        self.0.reset();
        self.0.tick(self.0.duration());
    }

    pub fn tick(&mut self, dt: f32) -> &Self {
        self.0.tick(dt);
        self
    }

    pub fn available(&self) -> bool {
        self.0.finished()
    }
}

impl Default for AnimTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(0.1, true))
    }
}

pub struct PlayerAnimEvent {
    pub state: AnimState,
}

pub struct IsaacAnimations;

impl IsaacAnimations {
    fn animation_update(
        time: Res<Time>,
        mut query: Query<(&mut TextureAtlasSprite, &mut Animation, &mut AnimTimer)>,
    ) {
        for (mut sprite, mut animation, mut timer) in query.iter_mut() {
            if timer.available() {
                sprite.index = animation.next_frame();
            }
            timer.tick(time.delta_seconds());
        }
    }

    fn player_animation(
        mut reader: Local<EventReader<PlayerAnimEvent>>,
        actions: ChangedRes<Actions<Action>>,
        anim_events: Res<Events<PlayerAnimEvent>>,
        mut query: Query<(&mut Transform, &mut Animation, &mut AnimTimer), With<Player>>,
    ) {
        for (mut transform, mut animation, mut timer) in query.iter_mut() {
            if let Some(Action::Shoot(direction)) = actions.get(Action::Shoot) {
                if direction.x > f32::EPSILON {
                    transform.scale.x = ZOOM;
                } else if direction.x < -f32::EPSILON {
                    transform.scale.x = -ZOOM;
                }
            } else if let Some(Action::Move(direction)) = actions.get(Action::Move) {
                if direction.x > f32::EPSILON {
                    transform.scale.x = ZOOM;
                } else if direction.x < -f32::EPSILON {
                    transform.scale.x = -ZOOM;
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
        app.add_event::<PlayerAnimEvent>()
            .add_system(Self::player_animation.system())
            .add_system(Self::animation_update.system());
    }
}
