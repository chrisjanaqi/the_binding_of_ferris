use crate::components::*;
use ggez::graphics;

pub type Player = (Translation, Velocity, Size, Rotation, Movement, TagPlayer);

pub struct PlayerMesh(pub graphics::Mesh);
// pub struct TearMesh(pub Mesh);

pub struct DeltaTime(pub f32);

impl PlayerMesh {
    pub fn new(ctx: &mut ggez::Context) -> Self {
        Self(
            graphics::MeshBuilder::new()
                .circle(
                    graphics::DrawMode::stroke(2.0),
                    [0.0, 0.0],
                    20.0,
                    0.1,
                    graphics::Color::from_rgb(52, 152, 219),
                )
                .build(ctx)
                .expect("Could not create player mesh"),
        )
    }
}
