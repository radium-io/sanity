use std::fmt::Debug;

use amethyst::{
    core::math::Point3,
    derive::SystemDesc,
    ecs::{Join, Read, ReadStorage, System, WriteStorage},
    input::InputHandler,
    input::StringBindings,
    tiles::MortonEncoder2D,
    tiles::{Map, TileMap},
    winit,
};
use amethyst::{ecs::SystemData, tiles::MapStorage};
use sanity_lib::{assets::Pairs, tile::RoomTile};

#[derive(SystemDesc, Default)]
pub struct SaveSystem {
    saving: bool,
}

impl<'s> System<'s> for SaveSystem {
    // The same BindingTypes from the InputBundle needs to be inside the InputHandler
    type SystemData = (
        ReadStorage<'s, TileMap<RoomTile>>,
        Read<'s, InputHandler<StringBindings>>,
    );

    fn run(&mut self, (tilemaps, input): Self::SystemData) {
        let mut p = Pairs::default();

        for tilemap in (&tilemaps).join() {
            let dim = tilemap.dimensions();
            for y in 0..dim.y {
                for x in 0..dim.x {
                    if let Some(t) = tilemap.get(&Point3::new(x, y, 0)) {
                        for s in t.candidates.s.as_slice() {
                            p.ns.push((t.sprite.unwrap(), *s));
                        }

                        for e in t.candidates.e.as_slice() {
                            p.we.push((t.sprite.unwrap(), *e))
                        }
                    }
                }
            }
        }

        if input.key_is_down(winit::VirtualKeyCode::S) && !self.saving {
            let s = ron::ser::to_string_pretty(&p, ron::ser::PrettyConfig::default()).unwrap();
            std::fs::write("asdf.pairs.ron", s);
        }
    }
}
