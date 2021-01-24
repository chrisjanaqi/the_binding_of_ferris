//! Items are entities that modify the behavior of the game when picked up by the player
//!
//! For an entity to be considered an item, they must have the [Item](Item) component.
//! When an Item is picked up, the entity become child of the player.
mod tiny_planet;

use bevy::{app::PluginGroupBuilder, prelude::*};
pub use tiny_planet::TinyPlanet;

/// Tag component identifying an item
// pub struct Item;

pub struct ItemPlugins;

pub struct ItemPickupEvent {
    pub player: Entity,
    pub item: Entity,
}

impl Plugin for ItemPlugins {
    fn build(&self, app: &mut AppBuilder) {
        app.add_event::<ItemPickupEvent>();
    }
}

impl PluginGroup for ItemPlugins {
    fn build(&mut self, group: &mut PluginGroupBuilder) {
        group.add(Self).add(TinyPlanet);
    }
}
