extern crate amethyst;

use amethyst::{
    core::math::Point3,
    ecs::{World, WorldExt},
    prelude::*,
    renderer::{plugins::RenderFlat2D, types::DefaultBackend, RenderingBundle},
    tiles::{RenderTiles2D, Tile},
};

#[derive(Clone, Default)]
pub struct SimpleTile;
impl Tile for SimpleTile {
    fn sprite(&self, coords: Point3<u32>, _: &World) -> Option<usize> {
        format!("{:?}", coords);
        let map = [
            [0, 1, 0, 1, 0, 1, 1, 0, 1, 0, 1, 1, 0, 1, 0, 1, 1, 0, 1, 0],
            [0, 1, 0, 1, 0, 1, 1, 0, 1, 0, 1, 1, 0, 1, 0, 1, 1, 0, 1, 0],
        ];
        Some(map[coords.y as usize][coords.x as usize])
    }
}
