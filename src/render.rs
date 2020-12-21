use bevy::{
    prelude::*,
    reflect::TypeUuid,
    render::{
        mesh::shape,
        pipeline::{PipelineDescriptor, RenderPipeline},
        render_graph::{base, AssetRenderResourcesNode, RenderGraph},
        renderer::RenderResources,
        shader::ShaderStages,
    },
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
        command: &mut Commands,
        asset_server: ResMut<AssetServer>,
        mut pipelines: ResMut<Assets<PipelineDescriptor>>,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<WaterCaustics>>,
        mut render_graph: ResMut<RenderGraph>,
    ) {
        let pipeline_handle = pipelines.add(PipelineDescriptor::default_config(ShaderStages {
            vertex: asset_server.load::<Shader, _>("shaders/water.vert"),
            fragment: Some(asset_server.load::<Shader, _>("shaders/water.frag")),
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
        log::debug!("Test");
        command
            .spawn(MeshBundle {
                mesh: meshes.add(Mesh::from(shape::Quad::new(Vec2::new(1920.0, 1080.0)))),
                render_pipelines: RenderPipelines::from_pipelines(vec![RenderPipeline::new(
                    pipeline_handle,
                )]),
                transform: Transform::from_translation(Vec3::new(0.0, 0.0, 1.0)),
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
