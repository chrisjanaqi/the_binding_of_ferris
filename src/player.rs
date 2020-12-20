use crate::animation::*;
use crate::input::*;
use crate::physic::*;
use bevy::prelude::*;

pub struct Player;

pub struct IsaacPlayer;

impl IsaacPlayer {
    fn player_movement(
        mut animation_events: ResMut<Events<PlayerAnimEvent>>,
        actions: Res<Actions<Action>>,
        mut query: Query<With<Player, &mut Movement>>,
    ) {
        use Action::*;
        for mut movement in query.iter_mut() {
            if let Some(Move(direction)) = actions.get(Move) {
                movement.direction = Some(*direction);
            } else {
                movement.direction = None;
            }

            if actions.just_started(Move).is_some() {
                animation_events.send(PlayerAnimEvent {
                    state: AnimState::Move(AnimOrientation::Side),
                });
            } else if actions.just_finished(Move).is_some() {
                animation_events.send(PlayerAnimEvent {
                    state: AnimState::Idle(AnimOrientation::Side),
                });
            }
        }
    }
}

impl Plugin for IsaacPlayer {
    fn build(&self, app: &mut AppBuilder) {
        app.add_systems(vec![Self::player_movement.system()]);
    }
}
