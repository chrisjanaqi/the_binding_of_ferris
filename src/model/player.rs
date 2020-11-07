use crate::components::*;
use ggez::graphics;

pub type Player = (
    Translation,
    Velocity,
    Size,
    Rotation,
    WorldToScreen,
    Movement,
    PlayerAction,
    PlayerStats,
    TagPlayer,
);

pub struct PlayerMesh(pub graphics::Image);

impl PlayerMesh {
    pub fn new(ctx: &mut ggez::Context) -> Self {
        let image =
            graphics::Image::new(ctx, "/assets/player.png").expect("Could not find player asset");
        Self(image)
    }
}
