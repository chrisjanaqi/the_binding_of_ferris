use crate::animation::*;
use crate::input::*;
use crate::physic::*;
use crate::Materials;
use bevy::prelude::*;

pub struct Player {
    pub attack_cooldown: Timer,
    pub attack_speed: f32,
    pub attack_lifetime: f32,
}

pub struct ShootEvent {
    pub parent: Entity,
    pub direction: Vec2,
    pub speed: f32,
    pub lifetime: f32,
}

#[derive(Debug, Default, Copy, Clone)]
pub struct TearTag;

pub struct IsaacPlayer;

impl IsaacPlayer {
    fn player_movement(
        mut animation_events: ResMut<Events<PlayerAnimEvent>>,
        actions: Res<Actions<Action>>,
        mut query: Query<With<Player, &mut Movement>>,
    ) {
        use crate::Action::*;
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

    fn shooting(
        time: Res<Time>,
        actions: Res<Actions<Action>>,
        mut shoot_events: ResMut<Events<ShootEvent>>,
        mut animation_events: ResMut<Events<PlayerAnimEvent>>,
        mut query: Query<(Entity, &mut Player)>,
    ) {
        use crate::Action::*;

        for (e, mut player) in query.iter_mut() {
            player.attack_cooldown.tick(time.delta_seconds);
            if let Some(Shoot(direction)) = actions.get(Shoot) {
                if player.attack_cooldown.finished {
                    // println!("Shoot event emitted");
                    player.attack_cooldown.reset();
                    shoot_events.send(ShootEvent {
                        parent: e,
                        direction: direction.normalize(),
                        speed: player.attack_speed,
                        lifetime: player.attack_lifetime,
                    });
                    animation_events.send(PlayerAnimEvent {
                        state: AnimState::Attack(AnimOrientation::Side),
                    });
                }
            }
        }
    }

    fn tear_spawn(
        mut command: Commands,
        materials: Res<Materials>,
        shoot_events: Res<Events<ShootEvent>>,
        mut event_reader: Local<EventReader<ShootEvent>>,
        query: Query<(&Transform, &Velocity)>,
    ) {
        for shoot in event_reader.iter(&shoot_events) {
            let direction = if shoot.direction.x().abs() > f32::EPSILON {
                Vec2::new(shoot.direction.x().signum(), 0.0)
            } else {
                Vec2::new(0.0, shoot.direction.y().signum())
            };
            // println!("{:?}", direction);
            if let Ok((transform, velocity)) = query.get(shoot.parent) {
                command
                    .spawn(SpriteSheetComponents {
                        transform: *transform,
                        texture_atlas: materials.tears.clone(),
                        ..Default::default()
                    })
                    .with(Velocity(direction * shoot.speed + 0.33 * velocity.0))
                    .with(Animation::from_length(3))
                    .with(Timer::from_seconds(shoot.lifetime, false))
                    .with(TearTag);
            }
        }
    }

    fn tear_despawn(mut command: Commands, query: Query<With<TearTag, (Entity, &Timer)>>) {
        for (entity, timer) in query.iter() {
            if timer.finished {
                // println!("Despawing tear {:?}", entity);
                command.despawn(entity);
            }
        }
    }
}

impl Plugin for IsaacPlayer {
    fn build(&self, app: &mut AppBuilder) {
        app.add_event::<ShootEvent>()
            .add_system(Self::player_movement.system())
            .add_system(Self::shooting.system())
            .add_system(Self::tear_spawn.system())
            .add_system(Self::tear_despawn.system());
    }
}
