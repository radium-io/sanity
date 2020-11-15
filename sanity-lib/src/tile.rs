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

#[derive(Clone, Debug)]
pub struct RoomTile {
    pub sprite: Option<TileSetIndex>,
    pub tint: palette::Srgba,
    pub candidates: Candidates,
    pub walkable: bool,
}

impl Default for RoomTile {
    fn default() -> Self {
        Self {
            sprite: None,
            tint: palette::Srgba::new(1., 1., 1., 1.),
            candidates: Candidates::default(),
            walkable: false,
        }
    }
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
        self.tint
    }
}
