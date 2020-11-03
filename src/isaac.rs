use amethyst::core::math::Translation;
use amethyst::{core::Transform, prelude::*, renderer::Camera};

type Data = StateData<'_, GameData<'_, '_>>;

pub struct Isaac;

impl SimpleState for Isaac {
    fn on_start(&mut self, data: Data) {
        let world = data.world;
        initialise_camera(world);
    }
}

fn initialise_camera(world: &mut World) {
    // Setup camera in a way that our screen covers whole arena and (0, 0) is in the bottom left.
    let mut transform = Transform::default();
    transform.set_translation_z(1.0);
    world
        .create_entity()
        .with(Camera::standard_2d(20.0, 20.0))
        .with(transform)
        .build();
}
