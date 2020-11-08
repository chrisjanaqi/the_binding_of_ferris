mod components;
mod model;
mod rendering;
mod systems;
mod utils;

// use crate::components::*;
use crate::model::*;
use crate::rendering::*;
use crate::systems::*;
use crate::utils::*;

use ggez::{event::*, graphics::*, timer::*, *};
use legion::*;
use log::debug;
use std::path::PathBuf;

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
            .add_system(shooting_system())
            .add_system(linear_simulation_system())
            .add_system(angular_simulation_system())
            .add_system(lifetime_system())
            .add_system(world_to_camera_system())
            .add_system(camera_to_screen_system())
            .build();
        let mut resources = Resources::default();
        resources.insert(DeltaTime(0.0));
        resources.insert(PlayerMesh::new(ctx));
        resources.insert(TearMesh::new(ctx));
        resources.insert(Inputs::default());
        resources.insert(KeyBindings::default());
        resources.insert(Camera::default());
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
        draw(
            ctx,
            &fps,
            DrawParam {
                dest: [20.0, 20.0].into(),
                ..Default::default()
            },
        )?;
        draw(ctx, &resolution, (Point::new(20.0, 45.0),))?;
        render_tears(ctx, &mut self.world, &mut self.resources)?;
        render_player(ctx, &mut self.world, &mut self.resources)?;
        present(ctx)
    }

    fn mouse_motion_event(&mut self, ctx: &mut Context, _: f32, _: f32, dx: f32, dy: f32) {
        if input::mouse::button_pressed(ctx, MouseButton::Left) {
            let mut camera = self.resources.get_mut::<Camera>().unwrap();
            camera.position.x += dx / Camera::WIDTH;
            camera.position.y += dy / Camera::WIDTH;
        }
    }

    fn mouse_wheel_event(&mut self, _ctx: &mut Context, _: f32, y: f32) {
        self.resources.get_mut::<Camera>().unwrap().zoom *= (y * 0.5).exp();
    }

    fn resize_event(&mut self, ctx: &mut Context, width: f32, height: f32) {
        debug!("Resized screen to {}, {}", width, height);
        let aspect_ratio = width / height;

        let target_height = 1080.0;
        let target_width = 1920.0;
        let vstrip = (Camera::WIDTH / aspect_ratio - target_height).max(0.0);
        let hstrip = (Camera::HEIGHT * aspect_ratio - target_width).max(0.0);

        let rect = Rect::new(
            -hstrip * 0.5,
            -vstrip * 0.5,
            Camera::WIDTH + hstrip,
            Camera::HEIGHT + vstrip,
        );
        set_screen_coordinates(ctx, rect).expect("Cannot resize screen");
    }
}

fn main() -> GameResult<()> {
    env_logger::init();
    let resources =
        PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap_or_default()).join("resources");

    let (mut ctx, mut event_loop) = ContextBuilder::new("isaac-tears", "ergo_games")
        .add_resource_path(&resources)
        .build()?;

    let mut game = Isaac::new(&mut ctx);
    if let Err(e) = event::run(&mut ctx, &mut event_loop, &mut game) {
        println!("Error occurred: {}", e);
    }
    Ok(())
}
