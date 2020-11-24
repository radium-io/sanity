use amethyst::{
    assets::{AssetStorage, Handle, Loader, ProgressCounter, RonFormat},
    core::{
        math::{Point2, Point3, Vector3},
        Named, Parent, Transform,
    },
    ecs::prelude::*,
    input::{is_close_requested, is_key_down},
    prelude::*,
    renderer::{
        camera::Camera,
        debug_drawing::{DebugLines, DebugLinesComponent, DebugLinesParams},
        palette::Srgba,
        Transparent,
    },
    tiles::TileMap,
    ui::UiCreator,
    window::ScreenDimensions,
    winit,
};

#[derive(Default)]
pub struct GameOverState;

impl SimpleState for GameOverState {}
