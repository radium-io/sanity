extern crate amethyst;

use amethyst::{core::math::Point3, ecs::World, renderer::palette, tiles::Tile};

type TileSetIndex = usize;

#[derive(Clone, Default, Debug)]
pub struct Candidates {
    pub n: Vec<TileSetIndex>,
    pub e: Vec<TileSetIndex>,
    pub s: Vec<TileSetIndex>,
    pub w: Vec<TileSetIndex>,
}

#[derive(Clone, Debug, Default)]
pub struct RoomTile {
    pub sprite: Option<TileSetIndex>,
    pub visible: bool,
    pub candidates: Candidates,
    pub walkable: bool,
    pub tint: Option<palette::Srgba>,
}

impl Tile for RoomTile {
    fn sprite(&self, _: Point3<u32>, _: &World) -> Option<usize> {
        // TODO: based on type of sprite and world conditions this sprite could change
        // e.g. if sanity changes (world) and this is a wall, it could reveal a door!
        // this could be stored on struct or we can determine it later but we would need to know
        // how many doors are in the room (which means need ref to room)
        // also would probably have some animation for change
        self.sprite
    }

    fn tint(&self, _: Point3<u32>, _: &World) -> palette::Srgba {
        if let Some(tint) = self.tint {
            tint
        } else {
            if self.visible {
                palette::Srgba::new(1., 1., 1., 1.)
            } else {
                palette::Srgba::new(0.1, 0.1, 0.1, 0.9)
            }
        }
    }
}
