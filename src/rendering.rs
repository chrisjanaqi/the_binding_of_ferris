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
    for (transform, _) in query.iter(world) {
        let pos = Text::new(format!("Pos({:.1}, {:.1})", transform.t.x, transform.t.y));
        render_image(ctx, transform, mesh)?;
        draw(ctx, &pos, (Point::new(20.0, 70.0),))?;
    }
    Ok(())
}

pub fn render_tears(
    ctx: &mut Context,
    world: &mut World,
    resources: &mut Resources,
) -> GameResult<()> {
    let mut query: _ = <(&WorldToScreen, &TagTear)>::query();
    let mesh = &resources.get::<TearMesh>().unwrap().0;
    for (transform, _) in query.iter(world) {
        render_mesh(ctx, transform, mesh)?;
    }
    Ok(())
}

fn render_image(ctx: &mut Context, transform: &WorldToScreen, mesh: &Image) -> GameResult<()> {
    let size = transform.s;
    let params = DrawParam {
        dest: transform.t.into(),
        rotation: transform.r.0.into(),
        scale: [size, size].into(),
        offset: [0.5, 0.5].into(),
        ..DrawParam::default()
    };
    draw(ctx, mesh, params)
}

fn render_mesh(ctx: &mut Context, transform: &WorldToScreen, mesh: &Mesh) -> GameResult<()> {
    let size = transform.s;
    let params = DrawParam {
        dest: transform.t.into(),
        rotation: transform.r.0.into(),
        scale: [size, size].into(),
        ..DrawParam::default()
    };
    draw(ctx, mesh, params)
}
