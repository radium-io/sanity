use amethyst::{
    derive::SystemDesc,
    ecs::Join,
    ecs::{
        prelude::{Entity, System, SystemData, WriteStorage},
        ReadStorage,
    },
    tiles::Map,
    tiles::{MapStorage, TileMap},
    ui::{UiFinder, UiText},
};
use sanity_lib::tile::RoomTile;

#[derive(Default, SystemDesc)]
pub struct UISystem {
    south_list: Option<Entity>,
    east_list: Option<Entity>,
}

impl<'a> System<'a> for UISystem {
    type SystemData = (
        WriteStorage<'a, UiText>,
        UiFinder<'a>,
        ReadStorage<'a, TileMap<RoomTile>>,
        ReadStorage<'a, crate::state::edit::Selected>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut ui_text, finder, tilemaps, selected) = data;

        if self.south_list.is_none() {
            self.south_list = finder.find("south_list");
        }

        for (t, s) in (&tilemaps, &selected).join() {
            if let Some(e) = self.south_list {
                if let Some(ui) = ui_text.get_mut(e) {
                    if let Some(p) = s.0 {
                        ui.text = format!(
                            "{:?} \n south:{:?} \n east:{:?}",
                            p.x + p.y * t.dimensions().x,
                            t.get(&p).map(|t| t.candidates.s.clone()).unwrap_or(vec![]),
                            t.get(&p).map(|t| t.candidates.e.clone()).unwrap_or(vec![]),
                        );
                    }
                }
            }
        }
    }
}
