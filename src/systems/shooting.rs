use crate::components::*;
use crate::model::{DeltaTime, Tears};

use legion::systems::CommandBuffer;
use legion::*;
use log::debug;

#[system(for_each)]
pub fn shooting(
    action: &PlayerAction,
    stats: &mut PlayerStats,
    position: &Translation,
    velocity: &Velocity,
    _: &TagPlayer,
    ecb: &mut CommandBuffer,
    #[resource] delta: &DeltaTime,
) {
    let DeltaTime(dt) = *delta;
    stats.attack_cooldown.update(dt);
    let is_shooting = action.shoot.norm() > std::f32::EPSILON;
    if is_shooting && stats.attack_cooldown.available() {
        debug!("Creating Tear...");
        stats.attack_cooldown.start();
        let speed = action.shoot * stats.tear_speed;
        let t = Tears {
            transform: (*position, Rotation::default(), Size::default()),
            velocity: (Velocity(0.25 * velocity.0 + speed), Tears::ANGULAR_VELOCITY),
            stats: TearStats {
                damage: stats.attack_damage,
            },
            w2s: WorldToScreen::default(),
            lifetime: Lifetime(stats.tear_lifetime),
        };
        //*
        ecb.push((
            t.transform.0,
            t.transform.1,
            t.transform.2,
            t.w2s,
            t.velocity.0,
            t.velocity.1,
            t.stats,
            t.lifetime,
            TagTear,
        ));
        // */
    }
}
