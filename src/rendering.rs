use crate::components::*;
use crate::model::PlayerMesh;

use ggez::{graphics, Context, GameResult};
use legion::*;

pub fn render_player(
    ctx: &mut Context,
    world: &mut World,
    resources: &mut Resources,
) -> GameResult<()> {
    let mut query: _ = <(&Translation, &Rotation, &Size, &TagPlayer)>::query();
    let mesh = &resources.get::<PlayerMesh>().unwrap().0;
    query
        .iter(world)
        .map(|(translation, rotation, size, _)| {
            let params = graphics::DrawParam {
                dest: translation.0.into(),
                rotation: rotation.0.into(),
                scale: [size.0, size.0].into(),
                ..graphics::DrawParam::default()
            };
            graphics::draw(ctx, mesh, params)
        })
        .all(|res| res.is_ok());
    Ok(())
}
