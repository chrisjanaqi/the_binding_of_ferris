use crate::components::*;
use crate::model::*;
use crate::utils::*;

use ggez::{graphics::*, Context, GameResult};
use legion::*;

pub fn render_player(
    ctx: &mut Context,
    world: &mut World,
    resources: &mut Resources,
) -> GameResult<()> {
    let mut query: _ = <(&WorldToScreen, &TagPlayer)>::query();
    let mesh = &resources.get::<PlayerMesh>().unwrap().0;
    query
        .iter(world)
        .map(|(transform, _)| {
            let size = transform.s;
            let params = DrawParam {
                dest: transform.t.into(),
                rotation: transform.r.0.into(),
                scale: [size, size].into(),
                offset: [0.5, 0.5].into(),
                ..DrawParam::default()
            };
            let pos = Text::new(format!("Pos({:.1}, {:.1})", transform.t.x, transform.t.y));
            draw(ctx, mesh, params)?;
            draw(ctx, &pos, (Point::new(20.0, 70.0),))
        })
        .all(|res| res.is_ok());
    Ok(())
}
