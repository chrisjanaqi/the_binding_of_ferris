use ggez::nalgebra as na;
use ggez::{
    event::{self, EventHandler},
    graphics,
    timer::{delta, fps},
    Context, ContextBuilder, GameResult,
};
use legion::*;

type Point = na::Point2<f32>;
// type Vector = na::Vector2<f32>;

#[derive(Default)]
struct Isaac {
    world: World,
}

impl Isaac {
    pub fn new(ctx: &mut Context) -> Self {
        Isaac {
            world: World::default(),
        }
    }
}

impl EventHandler for Isaac {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        let dt = delta(ctx).as_secs_f64();
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::BLACK);
        let fps = graphics::Text::new(format!("{:.0}", fps(ctx)));
        graphics::draw(ctx, &fps, (Point::new(20.0, 20.0),))?;
        graphics::present(ctx)
    }
}

fn main() {
    // Make a Context and an EventLoop.
    let (mut ctx, mut event_loop) = ContextBuilder::new("isaac-tears", "ergo_games")
        .build()
        .unwrap();

    let mut game = Isaac::new(&mut ctx);
    match event::run(&mut ctx, &mut event_loop, &mut game) {
        Ok(_) => println!("Exited cleanly."),
        Err(e) => println!("Error occurred: {}", e),
    }
}
