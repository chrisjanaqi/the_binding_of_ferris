use bevy::{
    prelude::*,
    render::{
        mesh::shape,
        pipeline::{DynamicBinding, PipelineDescriptor, PipelineSpecialization, RenderPipeline},
        render_graph::{base, AssetRenderResourcesNode, RenderGraph},
        renderer::RenderResources,
        shader::{ShaderStage, ShaderStages},
    },
    type_registry::TypeUuid,
};

pub struct Materials {
    pub player: Handle<TextureAtlas>,
    pub tears: Handle<TextureAtlas>,
    pub ground: Handle<ColorMaterial>,
}

#[derive(RenderResources, Default, TypeUuid)]
#[uuid = "67e7955a-c67f-49ea-943d-5f49de2ca653"]
struct WaterCaustics {
    diffuse: Color,
    highlight: Color,
    time: f32,
}

pub struct IsaacRendering;

impl IsaacRendering {
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

    fn pipeline_setup(
        mut command: Commands,
        mut pipelines: ResMut<Assets<PipelineDescriptor>>,
        mut shaders: ResMut<Assets<Shader>>,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<WaterCaustics>>,
        mut render_graph: ResMut<RenderGraph>,
    ) {
        let vertex_shader = std::fs::read_to_string("assets/shaders/water.vert").unwrap();
        let fragment_shader = std::fs::read_to_string("assets/shaders/water.frag").unwrap();
        let pipeline_handle = pipelines.add(PipelineDescriptor::default_config(ShaderStages {
            vertex: shaders.add(Shader::from_glsl(ShaderStage::Vertex, &vertex_shader)),
            fragment: Some(shaders.add(Shader::from_glsl(ShaderStage::Fragment, &fragment_shader))),
        }));

        render_graph.add_system_node(
            "water_caustics",
            AssetRenderResourcesNode::<WaterCaustics>::new(true),
        );

        render_graph
            .add_node_edge("water_caustics", base::node::MAIN_PASS)
            .unwrap();

        let material = materials.add(WaterCaustics {
            diffuse: Color::rgba(0.13, 0.59, 0.95, 1.0),
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

impl Plugin for IsaacRendering {
    fn build(&self, app: &mut AppBuilder) {
        app.add_asset::<WaterCaustics>()
            .add_startup_system(Self::pipeline_setup.system())
            .add_system(Self::caustics_update.system());
    }
}
