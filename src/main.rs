mod behaviors;
mod components;
mod factories;
mod model;
mod rendering;
mod systems;

use crate::components::*;
use crate::model::*;
use crate::rendering::*;

use ggez::nalgebra as na;
use ggez::{conf::*, event::*, graphics::*, timer::*, *};
use legion::*;

type Point = na::Point2<f32>;
// type Vector = na::Vector2<f32>;

struct DeltaTime(f32);

struct Isaac {
    world: World,
    schedule: Schedule,
    resources: Resources,
}

impl Isaac {
    pub fn new(ctx: &mut Context) -> Self {
        let schedule = Schedule::builder().build();
        let mut resources = Resources::default();
        resources.insert(DeltaTime(0.0));
        resources.insert(PlayerMesh(
            MeshBuilder::new()
                .circle(
                    DrawMode::stroke(2.0),
                    Point::new(200.0, 200.0),
                    20.0,
                    0.1,
                    Color::from_rgb(52, 152, 219),
                )
                .build(ctx)
                .expect("Could not create player mesh"),
        ));
        let mut world = World::default();
        world.push((
            Translation::default(),
            Velocity::default(),
            Size(1.0),
            Rotation(0.0.into()),
            TagPlayer,
        ));
        Isaac {
            world,
            schedule,
            resources,
        }
    }
}

impl EventHandler for Isaac {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        let dt = delta(ctx).as_secs_f32();
        self.resources.insert(DeltaTime(dt));
        self.schedule.execute(&mut self.world, &mut self.resources);
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        clear(ctx, BLACK);

        let fps = Text::new(format!("{:.0} FPS", fps(ctx)));
        let resolution = {
            let screen = screen_coordinates(ctx);
            Text::new(format!("{}x{}", screen.w, screen.h))
        };
        draw(ctx, &fps, (Point::new(20.0, 20.0),))?;
        draw(ctx, &resolution, (Point::new(20.0, 45.0),))?;

        render_player(ctx, &mut self.world, &mut self.resources)?;
        present(ctx)
    }

    fn resize_event(&mut self, ctx: &mut Context, width: f32, height: f32) {
        println!("Resized screen to {}, {}", width, height);
        set_screen_coordinates(ctx, Rect::new(0.0, 0.0, width, height))
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
        Err(e) => println!("Error occurred: {}", e),
        _ => {}
    }
    Ok(())
}
