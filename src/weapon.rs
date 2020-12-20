use crate::animation::*;
use crate::input::*;
use crate::physic::*;
use crate::player::*;
use crate::render::Materials;

use bevy::prelude::*;

#[derive(Debug, Default, Copy, Clone)]
pub struct TearTag;

pub struct SpawnProjectileEvent {
    pub parent: Entity,
    pub direction: Vec2,
    pub speed: f32,
    pub lifetime: f32,
}

pub struct DespawnProjectileEvent(pub Entity);

#[derive(Default)]
pub struct TearWeapon {
    cooldown: Timer,
    speed: f32,
    lifetime: f32,
}

impl TearWeapon {
    pub fn new(cooldown: f32, speed: f32, lifetime: f32) -> Self {
        let mut timer = Timer::from_seconds(cooldown, false);
        timer.finished = true;
        timer.just_finished = true;
        Self {
            cooldown: timer,
            speed,
            lifetime,
        }
    }

    pub fn available(&mut self) -> bool {
        self.cooldown.finished
    }

    pub fn reset(&mut self) {
        self.cooldown.reset();
    }

    pub fn tick(&mut self, dt: f32) {
        self.cooldown.tick(dt);
    }

    fn update_weapon(
        time: Res<Time>,
        actions: Res<Actions<Action>>,
        mut shoot_events: ResMut<Events<SpawnProjectileEvent>>,
        mut animation_events: ResMut<Events<PlayerAnimEvent>>,
        mut query: Query<With<Player, (Entity, &mut TearWeapon)>>,
    ) {
        use Action::*;

        for (e, mut weapon) in query.iter_mut() {
            weapon.tick(time.delta_seconds);

            if let Some(Shoot(direction)) = actions.get(Shoot) {
                if weapon.available() {
                    weapon.reset();
                    shoot_events.send(SpawnProjectileEvent {
                        parent: e,
                        direction: direction.normalize(),
                        speed: weapon.speed,
                        lifetime: weapon.lifetime,
                    });
                    animation_events.send(PlayerAnimEvent {
                        state: AnimState::Attack(AnimOrientation::Side),
                    });
                }
            }
        }
    }

    fn update_projectile(
        mut projectile_events: ResMut<Events<DespawnProjectileEvent>>,
        query: Query<With<TearTag, (Entity, &Timer)>>,
    ) {
        let events: Vec<_> = query
            .iter()
            .filter(|(_, timer)| timer.finished)
            .map(|(entity, _)| DespawnProjectileEvent(entity))
            .collect();
        projectile_events.extend(events.into_iter());
    }

    fn spawn(
        mut command: Commands,
        materials: Res<Materials>,
        projectile_events: Res<Events<SpawnProjectileEvent>>,
        mut event_reader: Local<EventReader<SpawnProjectileEvent>>,
        query: Query<(&Transform, &Velocity)>,
    ) {
        for shoot in event_reader.iter(&projectile_events) {
            let direction = if shoot.direction.x().abs() > f32::EPSILON {
                Vec2::new(shoot.direction.x().signum(), 0.0)
            } else {
                Vec2::new(0.0, shoot.direction.y().signum())
            };

            if let Ok((transform, velocity)) = query.get(shoot.parent) {
                command
                    .spawn(SpriteSheetComponents {
                        transform: Transform {
                            translation: transform.translation,
                            scale: Vec3::splat(ZOOM),
                            ..Default::default()
                        },
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

    fn despawn(
        mut command: Commands,
        mut event_reader: Local<EventReader<DespawnProjectileEvent>>,
        despawn_events: Res<Events<DespawnProjectileEvent>>,
    ) {
        for DespawnProjectileEvent(entity) in event_reader.iter(&despawn_events) {
            command.despawn(*entity);
        }
    }
}

pub struct IsaacWeapons;

impl Plugin for IsaacWeapons {
    fn build(&self, app: &mut AppBuilder) {
        app.add_event::<SpawnProjectileEvent>()
            .add_event::<DespawnProjectileEvent>()
            .add_systems(vec![
                TearWeapon::update_weapon.system(),
                TearWeapon::update_projectile.system(),
                TearWeapon::spawn.system(),
                TearWeapon::despawn.system(),
            ]);
    }
}
