use crate::animation::*;
use crate::input::*;
use crate::physic::*;
use crate::player::*;
use crate::weapons::*;

use bevy::prelude::*;

#[derive(Bundle)]
pub struct TearProjectileBundle {
    pub velocity: Velocity,
    pub lifetime: Timer,
    pub tag: TearTag,
}

#[derive(Default)]
pub struct TearWeapon {
    cooldown: Timer,
    speed: f32,
    lifetime: f32,
}

impl TearWeapon {
    pub fn new(cooldown: f32, speed: f32, lifetime: f32) -> Self {
        let mut timer = Timer::from_seconds(cooldown, false);
        timer.tick(cooldown);

        Self {
            cooldown: timer,
            speed,
            lifetime,
        }
    }

    pub fn available(&mut self) -> bool {
        self.cooldown.finished()
    }

    pub fn reset(&mut self) {
        self.cooldown.reset();
    }

    pub fn tick(&mut self, dt: f32) {
        self.cooldown.tick(dt);
    }

    fn update(
        time: Res<Time>,
        actions: Res<Actions<Action>>,
        mut shoot_events: ResMut<Events<SpawnProjectileEvent>>,
        mut animation_events: ResMut<Events<PlayerAnimEvent>>,
        mut query: Query<(Entity, &mut TearWeapon), With<Player>>,
    ) {
        use Action::*;

        for (e, mut weapon) in query.iter_mut() {
            weapon.tick(time.delta_seconds());

            if !weapon.available() {
                continue;
            }

            if let Some(Shoot(direction)) = actions.get(Shoot) {
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

impl Plugin for TearWeapon {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(Self::update.system());
    }
}
