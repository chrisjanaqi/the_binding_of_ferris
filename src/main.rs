mod animation;
mod attribute;
mod init;
mod input;
mod items;
mod physic;
mod player;
mod render;
mod ui;
mod weapon;

use crate::animation::IsaacAnimations;
use crate::attribute::IsaacAttributes;
use crate::init::IsaacInit;
use crate::input::IsaacInputs;
use crate::items::IsaacItems;
use crate::physic::IsaacPhysic;
use crate::player::IsaacPlayer;
use crate::render::IsaacRendering;
use crate::ui::IsaacUI;
use crate::weapon::IsaacWeapons;

use bevy::input::system::exit_on_esc_system;
use bevy::prelude::*;

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

fn main() {
    env_logger::init();
    App::build()
        .add_resource(WindowDescriptor {
            title: "Isaac's Tears".to_string(),
            vsync: false,
            ..Default::default()
        })
        .add_resource(ClearColor(Color::rgb(0.16, 0.16, 0.16)))
        .add_plugins(DefaultPlugins)
        .add_plugin(IsaacInit)
        .add_plugin(IsaacAttributes)
        .add_plugin(IsaacInputs)
        .add_plugin(IsaacAnimations)
        .add_plugin(IsaacPhysic)
        .add_plugin(IsaacPlayer)
        .add_plugin(IsaacWeapons)
        .add_plugin(IsaacRendering)
        .add_plugin(IsaacItems)
        .add_plugin(IsaacUI)
        .add_system(exit_on_esc_system.system())
        .run();
}
