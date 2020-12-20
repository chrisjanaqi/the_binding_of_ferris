use crate::animation::{Animation, ZOOM};
use crate::physic::*;
use crate::player::Player;
use crate::render::Materials;
use crate::weapon::TearWeapon;
use crate::FromRon;
use bevy::prelude::*;

pub struct IsaacInit;

impl IsaacInit {
    const STAGE: &'static str = "game_setup";

    fn texture_loading(
        mut command: Commands,
        asset_server: Res<AssetServer>,
        mut atlases: ResMut<Assets<TextureAtlas>>,
        mut textures: ResMut<Assets<ColorMaterial>>,
    ) {
        let player_handle = asset_server.load("scorpion.png");
        let tear_handle = asset_server.load("tear.png");
        let ground_handle = asset_server.load("ground.png");
        let player_atlas = TextureAtlas::from_grid(player_handle, Vec2::new(32.0, 32.0), 5, 5);
        let tear_atlas = TextureAtlas::from_grid(tear_handle, Vec2::new(8.0, 8.0), 3, 1);
        asset_server.watch_for_changes().unwrap();
        command.insert_resource(Materials {
            player: atlases.add(player_atlas),
            tears: atlases.add(tear_atlas),
            ground: textures.add(ground_handle.into()),
        });
    }

    fn camera_spawn(mut commands: Commands) {
        commands.spawn(Camera2dComponents::default());
    }

    fn player_spawn(mut command: Commands, materials: Res<Materials>) {
        let animation = Animation::from_file("assets/scorpion.ron")
            .map_err(|e| {
                println!("{:?}", e.to_string());
                e
            })
            .unwrap_or_default();
        command
            .spawn(SpriteSheetComponents {
                texture_atlas: materials.player.clone(),
                transform: Transform::from_scale(Vec3::splat(ZOOM)),
                ..Default::default()
            })
            .with(Player)
            .with(TearWeapon::new(0.5, 700.0, 1.2))
            .with(Velocity::default())
            .with(Movement {
                direction: None,
                acceleration: 5000.0,
                speed: 500.0,
                damping: 1500.0,
            })
            .with(animation);
    }

    fn ground_spawn(mut command: Commands, materials: Res<Materials>) {
        command.spawn(SpriteComponents {
            transform: Transform::from_scale(Vec3::splat(ZOOM)),
            material: materials.ground.clone(),
            ..Default::default()
        });
    }
}

impl Plugin for IsaacInit {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_systems(vec![
            Self::camera_spawn.system(),
            Self::texture_loading.system(),
        ])
        .add_startup_stage(Self::STAGE)
        .add_startup_systems_to_stage(
            Self::STAGE,
            vec![Self::ground_spawn.system(), Self::player_spawn.system()],
        );
    }
}
