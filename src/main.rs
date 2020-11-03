mod isaac;

use crate::isaac::Isaac;
use amethyst::{
    prelude::*,
    renderer::{types::DefaultBackend, RenderFlat2D, RenderToWindow, RenderingBundle},
    utils::application_root_dir,
};

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());
    let app_root = application_root_dir()?;
    let config_dir = app_root.join("config");
    let asset_dir = app_root.join("assets");

    let game_data = GameDataBuilder::default().with_bundle(
        RenderingBundle::<DefaultBackend>::new()
            .with_plugin(
                RenderToWindow::from_config_path(config_dir.join("display.ron"))?
                    .with_clear([0.00196, 0.23726, 0.21765, 1.0]),
            )
            .with_plugin(RenderFlat2D::default()),
    )?;
    let mut game = Application::new(asset_dir, Isaac, game_data)?;
    game.run();

    println!("hello world!");
    Ok(())
}
