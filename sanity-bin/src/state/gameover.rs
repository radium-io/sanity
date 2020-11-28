use amethyst::{
    input::{is_close_requested, is_key_down},
    prelude::*,
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
            } else if is_key_down(&event, winit::VirtualKeyCode::R) {
                Trans::Pop
            } else {
                Trans::None
            }
        } else {
            Trans::None
        }
    }
}
