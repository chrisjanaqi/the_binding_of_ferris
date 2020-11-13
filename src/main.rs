mod inputs;

use crate::inputs::*;
use bevy::{input::system::exit_on_esc_system, prelude::*};

use crate::inputs::Direction;
// use std::path::PathBuf;

struct IsaacInit;

impl IsaacInit {
    const STAGE: &'static str = "game_setup";

    fn camera(mut commands: Commands) {
        commands.spawn(Camera2dComponents::default());
    }

    fn player(
        mut commands: Commands,
        asset_server: Res<AssetServer>,
        mut atlases: ResMut<Assets<TextureAtlas>>,
    ) {
        let player_handle = asset_server.load("fox-run.png");
        let player_atlas = TextureAtlas::from_grid(player_handle, Vec2::new(24.0, 24.0), 6, 1);
        commands
            .spawn(SpriteSheetComponents {
                texture_atlas: atlases.add(player_atlas),
                transform: Transform::from_scale(Vec3::splat(6.0)),
                ..Default::default()
            })
            .with(Player)
            .with(Velocity::default())
            .with(Movement {
                direction: None,
                acceleration: 5000.0,
                speed: 500.0,
                damping: 1500.0,
            })
            .with(Timer::from_seconds(0.1, true));
    }
}

impl Plugin for IsaacInit {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_systems(vec![Self::camera.system()])
            .add_startup_stage(Self::STAGE)
            .add_startup_systems_to_stage(Self::STAGE, vec![Self::player.system()]);
    }
}

struct Player;

#[derive(Debug, Default)]
struct Velocity(Vec2);

struct Movement {
    pub direction: Option<Vec2>,
    pub speed: f32,
    pub acceleration: f32,
    pub damping: f32,
}

fn player_movement(
    actions: ChangedRes<Input<Action>>,
    mut query: Query<With<Player, &mut Movement>>,
) {
    use crate::Action::*;
    use crate::Direction::*;

    let mut direction = Vec2::default();
    if actions.pressed(Move(Left)) {
        direction += Vec2::new(-1.0, 0.0);
    }
    if actions.pressed(Move(Right)) {
        direction += Vec2::new(1.0, 0.0);
    }
    if actions.pressed(Move(Up)) {
        direction += Vec2::new(0.0, 1.0);
    }
    if actions.pressed(Move(Down)) {
        direction += Vec2::new(0.0, -1.0);
    }

    let is_moving = direction.length_squared() > f32::EPSILON;
    for mut movement in query.iter_mut() {
        movement.direction = if is_moving {
            Some(direction.normalize())
        } else {
            None
        };
    }
}

fn moving(time: Res<Time>, mut query: Query<(&Movement, &mut Velocity)>) {
    let dt = time.delta_seconds;
    for (movement, mut velocity) in query.iter_mut() {
        let is_moving = movement.direction.is_some();
        let target = movement.speed * movement.direction.unwrap_or_default();
        let current = velocity.0;
        let error = target - current;
        // println!("Target: {:?}", target);
        let norm = error.length();
        if norm > f32::EPSILON {
            let coeff = if is_moving {
                movement.acceleration
            } else {
                movement.damping
            };

            velocity.0 = if norm <= coeff * dt {
                target
            } else {
                current + coeff * dt / norm * error
            };
        }
    }
}

fn physics(time: Res<Time>, mut query: Query<(&Velocity, &mut Transform)>) {
    let dt = time.delta_seconds;
    for (velocity, mut transform) in query.iter_mut() {
        transform.translation += velocity.0.extend(0.0) * dt;
        let scale = transform.scale.x().abs();
        if velocity.0.x() > f32::EPSILON {
            *transform.scale.x_mut() = scale;
        } else if velocity.0.x() < -f32::EPSILON {
            *transform.scale.x_mut() = -scale;
        }
    }
}

fn animation(
    sprite_sheets: Res<Assets<TextureAtlas>>,
    mut query: Query<(&mut Timer, &mut TextureAtlasSprite, &Handle<TextureAtlas>)>,
) {
    for (timer, mut sprite, atlas_handle) in query.iter_mut() {
        if timer.finished {
            let sprite_sheet = sprite_sheets.get(atlas_handle).unwrap();
            sprite.index = (sprite.index + 1) % sprite_sheet.len() as u32;
        }
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
        .add_system(player_movement.system())
        .add_system(moving.system())
        .add_system(physics.system())
        .add_system(animation.system())
        .add_system(exit_on_esc_system.system())
        .run();
}
