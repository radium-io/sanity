use amethyst::{
    core::math::Point3,
    derive::SystemDesc,
    ecs::{
        prelude::{System, SystemData, WriteStorage},
        Join, ReadStorage,
    },
    tiles::{Map, MapStorage, TileMap},
};
use bracket_pathfinding::prelude::{field_of_view_set, Point};
use sanity_lib::{map::SanityMap, tile::RoomTile};

#[derive(Default, SystemDesc)]
pub struct VisibilitySystem {}

impl<'a> System<'a> for VisibilitySystem {
    type SystemData = (
        WriteStorage<'a, TileMap<RoomTile>>,
        ReadStorage<'a, crate::component::Player>,
    );

    fn run(&mut self, (mut tilemaps, players): Self::SystemData) {
        for player in (&players).join() {
            for tilemap in (&mut tilemaps).join() {
                let dim = tilemap.dimensions().clone();
                let clone = tilemap.clone();
                let my_map = SanityMap(&clone);
                let fov = field_of_view_set(player.pos(), 4, &my_map);

                for x in 0..dim.x {
                    for y in 0..dim.y {
                        let p = Point::new(x, y);
                        if let Some(tile) = tilemap.get_mut(&Point3::new(x, y, 0)) {
                            tile.visible = fov.contains(&p);
                        }
                    }
                }
            }
        }
    }
}
