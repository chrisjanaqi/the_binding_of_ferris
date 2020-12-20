mod animation_graph;
mod state;
pub use animation_graph::Animation;
pub use state::{AnimOrientation, AnimState};

use crate::input::*;
use crate::player::Player;

use bevy::prelude::*;

pub const ZOOM: f32 = 6.0;

#[derive(Debug, Default)]
pub struct AnimTimer(pub Timer);

impl AnimTimer {
    fn reset(&mut self) {
        self.0.finished = true;
        self.0.just_finished = true;
    }
}

pub struct PlayerAnimEvent {
    pub state: AnimState,
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
