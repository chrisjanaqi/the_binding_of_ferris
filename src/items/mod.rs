//! Items are entities that modify the behavior of the game when picked up by the player
//!
//! For an entity to be considered an item, they must have the [Item](Item) component.
//! When an Item is picked up, the entity become child of the player.

use bevy::prelude::*;

/// Tag component identifying an item
pub struct Item;

pub struct IsaacItems;

pub struct PickupItem {
    pub player: Entity,
    pub item: Entity,
}

impl IsaacItems {
    fn item_pickup(
        mut command: Commands,
        mut reader: Local<EventReader<PickupItem>>,
        pickup_events: Res<Events<PickupItem>>,
    ) {
        for &PickupItem { player, item } in reader.iter(&pickup_events) {
            command.push_children(player, &[item]);
        }
    }
}

impl Plugin for IsaacItems {
    fn build(&self, app: &mut AppBuilder) {
        app.add_event::<PickupItem>()
            .add_systems(vec![Self::item_pickup.system()]);
    }
}
