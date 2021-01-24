use crate::physic::*;
use crate::player::*;
use crate::render::Materials;
use crate::weapons::TearWeapon;
use crate::FromRon;
use crate::{animation::*, items};
use bevy::prelude::*;

pub struct InitPlugin;

impl InitPlugin {
    const STAGE: &'static str = "game_setup";

    fn texture_loading(
        command: &mut Commands,
        asset_server: Res<AssetServer>,
        mut atlases: ResMut<Assets<TextureAtlas>>,
        mut textures: ResMut<Assets<ColorMaterial>>,
    ) {
        asset_server.watch_for_changes().unwrap();

        let player_handle = asset_server.load("scorpion.png");
        let tear_handle = asset_server.load("tear.png");
        let ground_handle = asset_server.load("ground.png");
        let player_atlas = TextureAtlas::from_grid(player_handle, Vec2::new(32.0, 32.0), 5, 5);
        let tear_atlas = TextureAtlas::from_grid(tear_handle, Vec2::new(8.0, 8.0), 3, 1);
        command.insert_resource(Materials {
            player: atlases.add(player_atlas),
            tears: atlases.add(tear_atlas),
            ground: textures.add(ground_handle.into()),
        });
    }

    fn camera_spawn(command: &mut Commands) {
        command.spawn(Camera2dBundle::default());
    }

    fn player_spawn(command: &mut Commands, materials: Res<Materials>) {
        let animation = Animation::from_file("assets/scorpion.ron")
            .map_err(|e| {
                println!("{:?}", e.to_string());
                e
            })
            .unwrap_or_default();
        command
            .spawn(PlayerBundle {
                player: Player,
                weapon: TearWeapon::new(0.5, 700.0, 3.0),
                velocity: Default::default(),
                movement: Movement {
                    direction: None,
                    acceleration: 5000.0,
                    speed: 500.0,
                    damping: 1500.0,
                },
            })
            .with_bundle(SpriteSheetBundle {
                texture_atlas: materials.player.clone(),
                transform: Transform::from_scale(Vec3::splat(ZOOM)),
                ..Default::default()
            })
            .with_bundle(AnimationBundle {
                anim_timer: AnimTimer::new(10.0),
                animation,
            })
            .with(items::TinyPlanet);
    }

    fn ground_spawn(command: &mut Commands, materials: Res<Materials>) {
        command.spawn(SpriteBundle {
            transform: Transform::from_scale(Vec3::splat(ZOOM)),
            material: materials.ground.clone(),
            ..Default::default()
        });
    }
}

impl Plugin for InitPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(Self::camera_spawn.system())
            .add_startup_system(Self::texture_loading.system())
            .add_startup_stage(
                Self::STAGE,
                SystemStage::parallel()
                    .with_system(Self::ground_spawn.system())
                    .with_system(Self::player_spawn.system()),
            );
    }
}
