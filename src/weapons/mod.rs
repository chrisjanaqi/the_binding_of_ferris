mod default;

use crate::animation::*;
use crate::physic::*;
use crate::render::Materials;

use bevy::{app::PluginGroupBuilder, prelude::*};

pub use default::*;

#[derive(Debug, Default, Copy, Clone)]
pub struct TearTag;

pub struct SpawnProjectileEvent {
    pub parent: Entity,
    pub direction: Vec2,
    pub speed: f32,
    pub lifetime: f32,
}

pub struct DespawnProjectileEvent(pub Entity);

pub struct WeaponPlugins;

impl WeaponPlugins {
    fn update_projectile(
        time: Res<Time>,
        mut projectile_events: ResMut<Events<DespawnProjectileEvent>>,
        mut query: Query<(Entity, &mut Timer), With<TearTag>>,
    ) {
        let events: Vec<_> = query
            .iter_mut()
            .map(|(e, mut timer)| {
                timer.tick(time.delta_seconds());
                (e, timer)
            })
            .filter(|(_, timer)| timer.finished())
            .map(|(entity, _)| DespawnProjectileEvent(entity))
            .collect();
        projectile_events.extend(events.into_iter());
    }

    fn spawn(
        command: &mut Commands,
        materials: Res<Materials>,
        projectile_events: Res<Events<SpawnProjectileEvent>>,
        mut event_reader: Local<EventReader<SpawnProjectileEvent>>,
        query: Query<(&Transform, &Velocity)>,
    ) {
        for shoot in event_reader.iter(&projectile_events) {
            let direction = if shoot.direction.x.abs() > f32::EPSILON {
                Vec2::new(shoot.direction.x.signum(), 0.0)
            } else {
                Vec2::new(0.0, shoot.direction.y.signum())
            };

            if let Ok((transform, velocity)) = query.get(shoot.parent) {
                command
                    .spawn(TearProjectileBundle {
                        velocity: Velocity(direction * shoot.speed + 0.33 * velocity.0),
                        lifetime: Timer::from_seconds(shoot.lifetime, false),
                        tag: TearTag,
                    })
                    .with_bundle(SpriteSheetBundle {
                        transform: Transform {
                            translation: transform.translation,
                            scale: Vec3::splat(ZOOM),
                            ..Default::default()
                        },
                        texture_atlas: materials.tears.clone(),
                        ..Default::default()
                    })
                    .with_bundle(AnimationBundle {
                        animation: Animation::from_length(3),
                        anim_timer: AnimTimer::default(),
                    });
            }
        }
    }

    fn despawn(
        command: &mut Commands,
        mut event_reader: Local<EventReader<DespawnProjectileEvent>>,
        despawn_events: Res<Events<DespawnProjectileEvent>>,
    ) {
        for DespawnProjectileEvent(entity) in event_reader.iter(&despawn_events) {
            command.despawn(*entity);
        }
    }
}

impl Plugin for WeaponPlugins {
    fn build(&self, app: &mut AppBuilder) {
        app.add_event::<SpawnProjectileEvent>()
            .add_event::<DespawnProjectileEvent>()
            .add_system(Self::update_projectile.system())
            .add_system(Self::spawn.system())
            .add_system(Self::despawn.system());
    }
}

impl PluginGroup for WeaponPlugins {
    fn build(&mut self, group: &mut PluginGroupBuilder) {
        group.add(Self).add(TearWeapon::default());
    }
}
