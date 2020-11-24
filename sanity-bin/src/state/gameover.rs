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

use crate::gamedata::CustomGameData;

#[derive(Default)]
pub struct GameOverState;

impl<'a, 'b> State<crate::gamedata::CustomGameData<'a, 'b>, StateEvent> for GameOverState {
    fn update(
        &mut self,
        data: StateData<'_, CustomGameData<'_, '_>>,
    ) -> Trans<CustomGameData<'a, 'b>, StateEvent> {
        data.data.update(&data.world, false);

        Trans::None
    }

    fn handle_event(
        &mut self,
        data: StateData<'_, CustomGameData<'a, 'b>>,
        event: StateEvent,
    ) -> Trans<CustomGameData<'a, 'b>, StateEvent> {
        if let StateEvent::Window(event) = &event {
            if is_close_requested(&event) || is_key_down(&event, winit::VirtualKeyCode::Escape) {
                Trans::Quit
            } else {
                Trans::None
            }
        } else {
            Trans::None
        }
    }
}
