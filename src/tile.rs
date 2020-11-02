extern crate amethyst;

use amethyst::{core::math::Point3, ecs::World, tiles::Tile};

#[allow(dead_code)]
#[derive(Clone, Copy)]
pub enum TileType {
    Floor,
    WallN,
    WallE,
    WallS,
    WallW,
    CornerNE,
    CornerSE,
    CornerSW,
    CornerNW,
    DiagonalNE,
    DiagonalSE,
}

#[derive(Clone, Default)]
pub struct RoomTile {
    pub sprite: Option<TileType>,
}

impl Tile for RoomTile {
    fn sprite(&self, _: Point3<u32>, _: &World) -> Option<usize> {
        // TODO: based on type of sprite and world conditions this sprite could change
        // e.g. if sanity changes (world) and this is a wall, it could reveal a door!
        // this could be stored on struct or we can determine it later but we would need to know
        // how many doors are in the room (which means need ref to room)
        // also would probably have some animation for change
        self.sprite.map(|s| s as usize)
    }
}
