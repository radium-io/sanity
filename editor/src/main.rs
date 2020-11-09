use amethyst::{
    assets::Processor,
    core::frame_limiter::FrameRateLimitStrategy,
    core::transform::TransformBundle,
    input::{InputBundle, StringBindings},
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    tiles::RenderTiles2D,
    ui::{RenderUi, UiBundle},
    utils::application_root_dir,
    utils::fps_counter::FpsCounterBundle,
};

mod assets;
mod state;
mod system;
mod tile;

use amethyst::assets::{HotReloadBundle, HotReloadStrategy};

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;

    let game_data = GameDataBuilder::default()
        .with_bundle(HotReloadBundle::new(HotReloadStrategy::every(2)))?
        .with_bundle(TransformBundle::new())?
        .with_bundle(
            InputBundle::<StringBindings>::new()
                .with_bindings_from_file(&app_root.join("config/input.ron"))?,
        )?
        .with_bundle(UiBundle::<StringBindings>::new())?
        .with_bundle(FpsCounterBundle::default())?
        .with(
            system::fps::ExampleSystem::default(),
            "example_system",
            &["input_system"],
        )
        .with(Processor::<assets::Pairs>::new(), "", &[])
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(app_root.join("config/display_config.ron"))?
                        .with_clear([0.34, 0.36, 0.52, 1.0]),
                )
                .with_plugin(RenderUi::default())
                .with_plugin(RenderFlat2D::default())
                .with_plugin(RenderTiles2D::<tile::RoomTile>::default()),
        )?;

    let mut game = Application::build(app_root.join("assets"), state::room::RoomState::default())?
        .build(game_data)?;

    game.run();

    Ok(())
}
