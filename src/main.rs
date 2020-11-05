mod behaviors;
mod components;
mod inputs;
mod model;
mod rendering;
mod systems;
mod utils;

// use crate::components::*;
use crate::inputs::*;
use crate::model::*;
use crate::rendering::*;
use crate::systems::*;
use crate::utils::*;

use ggez::{conf::*, event::*, graphics::*, timer::*, *};
use legion::*;

struct Isaac {
    world: World,
    schedule: Schedule,
    resources: Resources,
}

impl Isaac {
    pub fn new(ctx: &mut Context) -> Self {
        let schedule = Schedule::builder()
            .add_system(player_input_system())
            .add_system(moving_system())
            .add_system(linear_simulation_system())
            .build();
        let mut resources = Resources::default();
        resources.insert(DeltaTime(0.0));
        resources.insert(PlayerMesh::new(ctx));
        resources.insert(Inputs::default());
        resources.insert(KeyBindings::default());
        let mut world = World::default();
        world.push(Player::default());
        Isaac {
            world,
            schedule,
            resources,
        }
    }

    fn update_inputs(&mut self, ctx: &Context) {
        self.resources.get_mut_or_default::<Inputs>().update(ctx);
    }
}

impl EventHandler for Isaac {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        let dt = delta(ctx).as_secs_f32();
        self.update_inputs(ctx);
        self.resources.insert(DeltaTime(dt));
        self.schedule.execute(&mut self.world, &mut self.resources);
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        clear(ctx, Color::from_rgb(33, 33, 33));

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
                .vsync(true)
                .samples(NumSamples::Four),
        )
        .window_mode(conf.window_mode.dimensions(1280.0, 720.0).resizable(true))
        .build()?;

    let mut game = Isaac::new(&mut ctx);
    if let Err(e) = event::run(&mut ctx, &mut event_loop, &mut game) {
        println!("Error occurred: {}", e);
    }
    Ok(())
}
