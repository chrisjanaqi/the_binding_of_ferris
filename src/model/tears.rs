use crate::components::*;
use crate::utils::Radian;
use ggez::graphics;

#[derive(Default)]
pub struct Tears {
    pub transform: (Translation, Rotation, Size),
    pub velocity: (Velocity, AngularVelocity),
    pub stats: TearStats,
    pub w2s: WorldToScreen,
    pub lifetime: Lifetime,
}

impl Tears {
    pub const ANGULAR_VELOCITY: AngularVelocity = AngularVelocity(Radian(1.0));
}

pub struct TearMesh(pub graphics::Mesh);

impl TearMesh {
    pub fn new(ctx: &mut ggez::Context) -> Self {
        Self(
            graphics::MeshBuilder::new()
                .circle(
                    graphics::DrawMode::stroke(3.0),
                    [0.0, 0.0],
                    15.0,
                    0.1,
                    graphics::WHITE,
                )
                .build(ctx)
                .unwrap(),
        )
    }
}
