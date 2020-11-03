mod behaviors;
mod components;

use ggez::conf::{Conf, NumSamples};
use ggez::graphics::Rect;
use ggez::nalgebra as na;
use ggez::{
    event::{self, EventHandler},
    graphics,
    timer::{delta, fps},
    Context, ContextBuilder, GameResult,
};
use legion::{Resources, Schedule, World};

type Point = na::Point2<f32>;
// type Vector = na::Vector2<f32>;

struct Isaac {
    world: World,
    schedule: Schedule,
    resources: Resources,
    dt: f32,
}

impl Isaac {
    pub fn new(_ctx: &mut Context) -> Self {
        let schedule = Schedule::builder().build();
        Isaac {
            world: World::default(),
            schedule,
            resources: Resources::default(),
            dt: 0.0,
        }
    }
}

impl EventHandler for Isaac {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        let dt = delta(ctx).as_secs_f32();
        self.dt = dt;
        self.schedule.execute(&mut self.world, &mut self.resources);
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::BLACK);

        graphics::draw(
            ctx,
            &graphics::Text::new(format!("{:.0}", fps(ctx))),
            (Point::new(20.0, 20.0),),
        )?;
        let screen = graphics::screen_coordinates(ctx);
        graphics::draw(
            ctx,
            &graphics::Text::new(format!("{:?}", screen)),
            (Point::new(20.0, 60.0),),
        )?;
        graphics::present(ctx)
    }

    fn resize_event(&mut self, ctx: &mut Context, width: f32, height: f32) {
        println!("Resized screen to {}, {}", width, height);
        graphics::set_screen_coordinates(ctx, Rect::new(0.0, 0.0, width, height))
            .expect("Cannot resize screen");
    }
}

fn main() -> GameResult<()> {
    // Make a Context and an EventLoop.
    let conf = Conf::new();
    let (mut ctx, mut event_loop) = ContextBuilder::new("isaac-tears", "ergo_games")
        .window_setup(
            conf.window_setup
                .title("Isaac's tears")
                .vsync(false)
                .samples(NumSamples::Four),
        )
        .window_mode(conf.window_mode.dimensions(1280.0, 720.0).resizable(true))
        .build()?;

    let mut game = Isaac::new(&mut ctx);
    match event::run(&mut ctx, &mut event_loop, &mut game) {
        Ok(_) => println!("Exited cleanly."),
        Err(e) => println!("Error occurred: {}", e),
    }
    Ok(())
}
