use amethyst::{
    core::Hidden,
    ecs::prelude::*,
    input::{is_close_requested, is_key_down},
    prelude::*,
    ui::UiCreator,
    ui::UiFinder,
    winit,
};

use crate::gamedata::CustomGameData;

#[derive(Default)]
pub struct GameOverState {
    message: Option<Entity>,
}

impl<'a, 'b> State<crate::gamedata::CustomGameData<'a, 'b>, StateEvent> for GameOverState {
    fn on_start(&mut self, data: StateData<'_, CustomGameData<'a, 'b>>) {
        if self.message.is_none() {
            data.world.exec(|mut creator: UiCreator<'_>| {
                self.message = Some(creator.create("ui/gameover.ron", ()));
            });
        }
    }

    fn on_stop(&mut self, data: StateData<'_, CustomGameData<'a, 'b>>) {
        data.world.exec(|entities: Entities<'_>| {
            entities.delete(self.message.unwrap());
        });
    }
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
