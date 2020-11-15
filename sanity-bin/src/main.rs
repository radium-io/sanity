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

mod state;
mod system;

use amethyst::assets::{HotReloadBundle, HotReloadStrategy};
use amethyst_utils::ortho_camera::CameraOrthoSystem;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;

    let resources = app_root.parent().unwrap().join("assets");
    let display_config = app_root.join("config/display_config.ron");
    let key_bindings_path = app_root.join("config/input.ron");

    let game_data = GameDataBuilder::default()
        .with(CameraOrthoSystem::default(), "ortho_camera_system", &[])
        .with_bundle(HotReloadBundle::new(HotReloadStrategy::every(2)))?
        .with_bundle(TransformBundle::new())?
        .with_bundle(
            InputBundle::<StringBindings>::new().with_bindings_from_file(&key_bindings_path)?,
        )?
        .with_bundle(UiBundle::<StringBindings>::new())?
        .with_bundle(FpsCounterBundle::default())?
        .with(system::fps::FPSSystem::default(), "fps_system", &[])
        .with(
            system::visibility::VisibilitySystem::default(),
            "visibility_system",
            &[],
        )
        .with(
            system::movement::MovementSystem::default(),
            "movement_system",
            &["input_system"],
        )
        .with(Processor::<sanity_lib::assets::Pairs>::new(), "", &[])
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config)?
                        .with_clear([0.34, 0.36, 0.52, 1.0]),
                )
                .with_plugin(RenderUi::default())
                .with_plugin(RenderFlat2D::default())
                .with_plugin(RenderTiles2D::<sanity_lib::tile::RoomTile>::default()),
        )?;

    let mut game = Application::build(resources, state::room::RoomState::default())?
        .with_frame_limit(FrameRateLimitStrategy::Yield, 101)
        .build(game_data)?;

    game.run();

    Ok(())
}
