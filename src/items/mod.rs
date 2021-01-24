//! Items are entities that modify the behavior of the game when picked up by the player
//!
//! For an entity to be considered an item, they must have the [Item](Item) component.
//! When an Item is picked up, the entity become child of the player.

use bevy::prelude::*;

use crate::{
    physic::{self, IsaacPhysic, Velocity},
    player::Player,
    weapon::TearTag,
};

/// Tag component identifying an item
// pub struct Item;

#[derive(Debug, Default, Clone, Copy)]
pub struct OrbitalTears;

impl OrbitalTears {
    const RADIUS_TARGET: f32 = 350.0;

    fn radius(pos: Vec2, center: Vec2) -> f32 {
        pos.distance(center)
    }

    fn orthogonal(v: Vec2) -> Vec2 {
        Vec2::new(-v.y, v.x)
    }

    fn update(
        mut tear_query: Query<(&Transform, &mut Velocity), With<TearTag>>,
        player_query: Query<&Transform, With<Player>>,
    ) {
        let dt = IsaacPhysic::FIXED_TIMESTEP as f32;
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

                let error = 1.0 - Self::radius(pos, player) / Self::RADIUS_TARGET;

                *velocity = (tangent + 60.0 * error * dt * radial).normalize() * speed;
            }
        }
    }
}

pub struct IsaacItems;

pub struct ItemPickupEvent {
    pub player: Entity,
    pub item: Entity,
}

impl Plugin for IsaacItems {
    fn build(&self, app: &mut AppBuilder) {
        app.add_event::<ItemPickupEvent>()
            .add_system_to_stage(physic::STAGE, OrbitalTears::update.system());
    }
}
