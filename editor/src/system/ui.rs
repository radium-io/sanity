use amethyst::{
    core::timing::Time,
    derive::SystemDesc,
    ecs::{
        prelude::{Entity, Read, System, SystemData, WriteStorage},
        ReadStorage,
    },
    tiles::TileMap,
    ui::{UiFinder, UiText},
    utils::fps_counter::FpsCounter,
};
use sanity_lib::tile::RoomTile;

#[derive(Default, SystemDesc)]
pub struct ExampleSystem {
    south_list: Option<Entity>,
    east_list: Option<Entity>,
}

impl<'a> System<'a> for ExampleSystem {
    type SystemData = (
        WriteStorage<'a, UiText>,
        UiFinder<'a>,
        ReadStorage<'a, TileMap<RoomTile>>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut ui_text, finder, tilemaps) = data;

        if self.south_list.is_none() {
            if let Some(e) = finder.find("south_list") {
                self.south_list = Some(e);
            }
        }

        if let Some(e) = self.south_list {
            if let Some(ui) = ui_text.get_mut(e) {
                ui.text = tilemaps.get()
            }
        }
    }
}
