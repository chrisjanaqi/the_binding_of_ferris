use bevy::prelude::*;

use crate::{
    physic::{self, Velocity},
    player::Player,
    weapons::TearTag,
};

#[derive(Debug, Default, Clone, Copy)]
pub struct TinyPlanet;

impl TinyPlanet {
    const RADIUS_TARGET: f32 = 350.0;

    fn orthogonal(v: Vec2) -> Vec2 {
        Vec2::new(-v.y, v.x)
    }

    fn update(
        mut tear_query: Query<(&Transform, &mut Velocity), With<TearTag>>,
        player_query: Query<&Transform, (With<Player>, With<TinyPlanet>)>,
    ) {
        let dt = physic::TIMESTEP as f32;
        if let Some(Transform { translation, .. }) = player_query.iter().next() {
            let player = translation.truncate();
            for (transform, mut velocity) in tear_query.iter_mut() {
                let velocity = &mut velocity.0;
                let pos = transform.translation.truncate();
                let radius = (pos - player).length();

                if radius < f32::EPSILON {
                    continue;
                }

                let radial = (pos - player) / radius;
                let tangent = Self::orthogonal(radial);
                let speed = velocity.length();

                let error = 1.0 - pos.distance(player) / Self::RADIUS_TARGET;

                *velocity = (tangent + 60.0 * error * dt * radial).normalize() * speed;
            }
        }
    }
}

impl Plugin for TinyPlanet {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_to_stage(physic::STAGE, Self::update.system());
    }
}
