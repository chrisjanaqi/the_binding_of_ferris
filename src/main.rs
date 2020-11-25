mod animation;
mod input;
mod physic;
mod player;

use crate::animation::*;
use crate::input::*;
use crate::physic::*;
use crate::player::*;
use bevy::{input::system::exit_on_esc_system, prelude::*};
// use std::path::PathBuf;

pub struct Materials {
    pub player: Handle<TextureAtlas>,
    pub tears: Handle<TextureAtlas>,
}

struct IsaacInit;

impl IsaacInit {
    const STAGE: &'static str = "game_setup";

    fn load_texture(
        mut command: Commands,
        asset_server: Res<AssetServer>,
        mut atlases: ResMut<Assets<TextureAtlas>>,
    ) {
        let player_handle = asset_server.load("scorpion.png");
        let tear_handle = asset_server.load("tear.png");
        let player_atlas = TextureAtlas::from_grid(player_handle, Vec2::new(32.0, 32.0), 5, 5);
        let tear_atlas = TextureAtlas::from_grid(tear_handle, Vec2::new(8.0, 8.0), 2, 1);
        command.insert_resource(Materials {
            player: atlases.add(player_atlas),
            tears: atlases.add(tear_atlas),
        });
    }

    fn camera(mut commands: Commands) {
        commands.spawn(Camera2dComponents::default());
    }

    fn player(mut commands: Commands, materials: Res<Materials>) {
        let animation = Animation::from_file("assets/scorpion.ron")
            .map_err(|e| {
                println!("{:?}", e.to_string());
                e
            })
            .unwrap_or_default();
        commands
            .spawn(SpriteSheetComponents {
                texture_atlas: materials.player.clone(),
                transform: Transform::from_scale(Vec3::splat(ZOOM)),
                ..Default::default()
            })
            .with(Player {
                attack_cooldown: Timer::from_seconds(0.5, false),
                attack_speed: 750.0,
                attack_lifetime: 2.0,
            })
            .with(Velocity::default())
            .with(Movement {
                direction: None,
                acceleration: 5000.0,
                speed: 500.0,
                damping: 1500.0,
            })
            .with(animation);
    }
}

impl Plugin for IsaacInit {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_systems(vec![Self::camera.system(), Self::load_texture.system()])
            .add_startup_stage(Self::STAGE)
            .add_startup_systems_to_stage(Self::STAGE, vec![Self::player.system()]);
    }
}

fn main() {
    env_logger::init();
    App::build()
        .add_resource(WindowDescriptor {
            title: "Isaac's Tears".to_string(),
            ..Default::default()
        })
        .add_resource(ClearColor(Color::rgb(0.12, 0.12, 0.12)))
        .add_plugins(DefaultPlugins)
        .add_plugin(IsaacInit)
        .add_plugin(IsaacInputs)
        .add_plugin(IsaacAnimations)
        .add_plugin(IsaacPhysic)
        .add_plugin(IsaacPlayer)
        .add_system(exit_on_esc_system.system())
        .run();
}
