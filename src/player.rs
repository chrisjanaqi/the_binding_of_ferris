use crate::input::*;
use crate::{AnimOrientation, AnimState, Animation, Movement};
use bevy::prelude::*;

pub struct Player {
    pub attack_cooldown: Timer,
}

pub struct ShootEvent {
    origin: Entity,
    direction: Vec2,
}

pub struct IsaacPlayer;

impl IsaacPlayer {
    fn player_action(
        actions: Res<Actions<Action>>,
        mut query: Query<With<Player, (&mut Movement, &mut Animation)>>,
    ) {
        use crate::Action::*;
        for (mut movement, mut animation) in query.iter_mut() {
            if let Some(Move(direction)) = actions.get(Move(Default::default())) {
                animation.set_state(AnimState::Move(AnimOrientation::Side));
                movement.direction = Some(*direction);
            } else {
                animation.set_state(AnimState::Idle(AnimOrientation::Side));
                movement.direction = None;
            }
        }
    }

    fn shooting(
        time: Res<Time>,
        _actions: Res<Actions<Action>>,
        mut shoot_events: ResMut<Events<ShootEvent>>,
        mut query: Query<(Entity, &mut Player)>,
    ) {
        // use crate::Action::*;
        let direction = Vec2::default();

        let is_shooting = direction.length_squared() > f32::EPSILON;

        for (e, mut player) in query.iter_mut() {
            player.attack_cooldown.tick(time.delta_seconds);
            if is_shooting && player.attack_cooldown.finished {
                println!("Shoot event emitted");
                player.attack_cooldown.reset();
                shoot_events.send(ShootEvent {
                    origin: e,
                    direction: direction.normalize(),
                });
            }
        }
    }
}

impl Plugin for IsaacPlayer {
    fn build(&self, app: &mut AppBuilder) {
        app.add_event::<ShootEvent>()
            .add_system(Self::player_action.system())
            .add_system(Self::shooting.system());
    }
}
