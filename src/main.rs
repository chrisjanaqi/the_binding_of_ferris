mod animation;
mod input;
mod physic;
mod player;
mod ui;

use crate::animation::*;
use crate::input::*;
use crate::physic::*;
use crate::player::*;
use crate::ui::*;

use bevy::prelude::*;
use bevy::{
    input::system::exit_on_esc_system,
    render::{
        mesh::shape,
        pipeline::{DynamicBinding, PipelineDescriptor, PipelineSpecialization, RenderPipeline},
        render_graph::{base, AssetRenderResourcesNode, RenderGraph},
        renderer::RenderResources,
        shader::{ShaderStage, ShaderStages},
    },
    type_registry::TypeUuid,
};
// use std::path::PathBuf;

pub struct Materials {
    pub player: Handle<TextureAtlas>,
    pub tears: Handle<TextureAtlas>,
    pub ground: Handle<ColorMaterial>,
}

#[derive(RenderResources, Default, TypeUuid)]
#[uuid = "67e7955a-c67f-49ea-943d-5f49de2ca653"]
pub struct WaterCaustics {
    diffuse: Color,
    highlight: Color,
    time: f32,
}

struct IsaacInit;

impl IsaacInit {
    const STAGE: &'static str = "game_setup";

    fn load_texture(
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

    fn pipeline_setup(
        mut command: Commands,
        mut pipelines: ResMut<Assets<PipelineDescriptor>>,
        mut shaders: ResMut<Assets<Shader>>,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<WaterCaustics>>,
        mut render_graph: ResMut<RenderGraph>,
    ) {
        if let Ok(vertex_shader) = std::fs::read_to_string("assets/shaders/water.vert") {
            if let Ok(fragment_shader) = std::fs::read_to_string("assets/shaders/water.frag") {
                let pipeline_handle =
                    pipelines.add(PipelineDescriptor::default_config(ShaderStages {
                        vertex: shaders.add(Shader::from_glsl(ShaderStage::Vertex, &vertex_shader)),
                        fragment: Some(
                            shaders.add(Shader::from_glsl(ShaderStage::Fragment, &fragment_shader)),
                        ),
                    }));

                render_graph.add_system_node(
                    "water_caustics",
                    AssetRenderResourcesNode::<WaterCaustics>::new(true),
                );

                render_graph
                    .add_node_edge("water_caustics", base::node::MAIN_PASS)
                    .unwrap();

                let material = materials.add(WaterCaustics {
                    diffuse: Color::rgb_u8(1, 87, 155),
                    highlight: Color::WHITE,
                    time: 0.0,
                });

                command
                    .spawn(MeshComponents {
                        mesh: meshes.add(Mesh::from(shape::Quad::new(Vec2::new(1920.0, 1080.0)))),
                        render_pipelines: RenderPipelines::from_pipelines(vec![
                            RenderPipeline::specialized(
                                pipeline_handle,
                                // NOTE: in the future you wont need to manually declare dynamic bindings
                                PipelineSpecialization {
                                    dynamic_bindings: vec![
                                        // Transform
                                        DynamicBinding {
                                            bind_group: 1,
                                            binding: 0,
                                        },
                                        // WaterCaustics_diffuse
                                        DynamicBinding {
                                            bind_group: 1,
                                            binding: 1,
                                        },
                                        // WaterCaustics_highlight
                                        DynamicBinding {
                                            bind_group: 1,
                                            binding: 2,
                                        },
                                        // WaterCaustics_dt
                                        DynamicBinding {
                                            bind_group: 1,
                                            binding: 3,
                                        },
                                    ],
                                    ..Default::default()
                                },
                            ),
                        ]),
                        transform: Transform::from_translation(Vec3::new(0.0, 0.0, 1.0)),
                        draw: Draw {
                            is_transparent: true,
                            ..Default::default()
                        },
                        ..Default::default()
                    })
                    .with(material);
            }
        }
    }

    fn camera(mut commands: Commands) {
        commands.spawn(Camera2dComponents::default());
    }

    fn player(mut command: Commands, materials: Res<Materials>) {
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
            .with(Player {
                attack_cooldown: Timer::from_seconds(0.5, false),
                attack_speed: 750.0,
                attack_lifetime: 1.2,
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

    fn ground(mut command: Commands, materials: Res<Materials>) {
        command.spawn(SpriteComponents {
            transform: Transform::from_scale(Vec3::splat(ZOOM)),
            material: materials.ground.clone(),
            ..Default::default()
        });
    }
}

impl Plugin for IsaacInit {
    fn build(&self, app: &mut AppBuilder) {
        app.add_asset::<WaterCaustics>()
            .add_startup_systems(vec![
                Self::camera.system(),
                Self::load_texture.system(),
                Self::pipeline_setup.system(),
            ])
            .add_startup_stage(Self::STAGE)
            .add_startup_systems_to_stage(
                Self::STAGE,
                vec![Self::ground.system(), Self::player.system()],
            );
    }
}

fn caustics_update(
    time: Res<Time>,
    mut water_caustics: ResMut<Assets<WaterCaustics>>,
    query: Query<&Handle<WaterCaustics>>,
) {
    for handle in query.iter() {
        if let Some(water) = water_caustics.get_mut(handle) {
            water.time = time.time_since_startup().as_secs_f32();
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
        .add_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_plugins(DefaultPlugins)
        .add_plugin(IsaacInit)
        .add_plugin(IsaacInputs)
        .add_plugin(IsaacAnimations)
        .add_plugin(IsaacPhysic)
        .add_plugin(IsaacPlayer)
        .add_plugin(IsaacUI)
        .add_system(caustics_update.system())
        .add_system(exit_on_esc_system.system())
        .run();
}
