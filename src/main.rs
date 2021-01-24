mod animation;
mod attribute;
mod init;
mod input;
mod items;
mod physic;
mod player;
mod render;
mod ui;
mod weapons;

use crate::animation::AnimationPlugin;
use crate::attribute::AttributesPlugin;
use crate::init::InitPlugin;
use crate::input::InputPlugin;
use crate::items::ItemPlugins;
use crate::physic::PhysicPlugin;
use crate::player::PlayerPlugin;
use crate::ui::UIPlugin;
use crate::weapons::WeaponPlugins;

use bevy::{input::system::exit_on_esc_system, prelude::*};

trait FromRon: Sized {
    fn from_file(path: &str) -> Result<Self, ron::Error>;
}

impl<T> FromRon for T
where
    T: serde::de::DeserializeOwned,
{
    fn from_file(path: &str) -> Result<T, ron::Error> {
        ron::from_str(&std::fs::read_to_string(path)?)
    }
}

#[bevy_main]
fn main() {
    App::build()
        .add_resource(WindowDescriptor {
            title: "Ferris's Tears".to_string(),
            vsync: false,
            ..Default::default()
        })
        .add_resource(ClearColor(Color::rgb(0.16, 0.16, 0.16)))
        .add_plugins(DefaultPlugins)
        .add_plugin(InitPlugin)
        .add_plugin(AttributesPlugin)
        .add_plugin(InputPlugin)
        .add_plugin(AnimationPlugin)
        .add_plugin(PhysicPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugins(WeaponPlugins)
        .add_plugins(ItemPlugins)
        .add_plugin(UIPlugin)
        .add_system(exit_on_esc_system.system())
        .run();
}
