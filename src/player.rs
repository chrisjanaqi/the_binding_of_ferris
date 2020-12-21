use crate::animation::*;
use crate::input::*;
use crate::physic::*;
use crate::weapon::*;
use bevy::prelude::*;

pub struct Player;

#[derive(Bundle)]
pub struct PlayerBundle {
    pub player: Player,
    pub weapon: TearWeapon,
    pub velocity: Velocity,
    pub movement: Movement,
}

pub struct IsaacPlayer;

impl IsaacPlayer {
    fn player_movement(
        mut animation_events: ResMut<Events<PlayerAnimEvent>>,
        actions: Res<Actions<Action>>,
        mut query: Query<&mut Movement, With<Player>>,
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
        app.add_system(Self::player_movement.system());
    }
}
