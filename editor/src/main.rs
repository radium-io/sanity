use amethyst::{
    assets::Processor,
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
};

use amethyst::assets::{HotReloadBundle, HotReloadStrategy};
use amethyst_utils::ortho_camera::CameraOrthoSystem;
use sanity_lib;
mod state;
mod system;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;
    let bindings_config = app_root.join("config").join("input.ron");

    let game_data = GameDataBuilder::default()
        .with(CameraOrthoSystem::default(), "ortho_camera_system", &[])
        .with_bundle(TransformBundle::new())?
        .with_bundle(HotReloadBundle::new(HotReloadStrategy::every(2)))?
        .with_bundle(
            InputBundle::<StringBindings>::new().with_bindings_from_file(bindings_config)?,
        )?
        .with_bundle(UiBundle::<StringBindings>::new())?
        .with(Processor::<sanity_lib::assets::Pairs>::new(), "", &[])
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(app_root.join("config/display_config.ron"))?
                        .with_clear([0.34, 0.36, 0.52, 1.0]),
                )
                .with_plugin(RenderUi::default())
                .with_plugin(RenderFlat2D::default())
                .with_plugin(RenderTiles2D::<sanity_lib::tile::RoomTile>::default()),
        )?
        .with(
            system::TileSelectSystem::default(),
            "tile_select_system",
            &["input_system"],
        )
        .with(system::SaveSystem::default(), "save_system", &[]);
    // allow args for ron and png
    let args: Vec<String> = std::env::args().collect();
    let assets_dir = app_root.parent().unwrap().join("assets");

    let mut game = match args.len() {
        4 => Application::build(
            assets_dir,
            crate::state::EditState {
                png: args[1].parse().unwrap(),
                ron: args[2].parse().unwrap(),
                pairs: args[3].parse().unwrap(),
                ..Default::default()
            },
        )?
        .build(game_data)?,
        _ => {
            Application::build(assets_dir, crate::state::LoadState::default())?.build(game_data)?
        }
    };

    game.run();

    Ok(())
}
