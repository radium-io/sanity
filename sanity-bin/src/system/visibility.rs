use amethyst::{
    core::{
        math::{Point3, Vector3},
        timing::Time,
        Transform,
    },
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
        ReadStorage<'a, Transform>,
    );

    fn run(&mut self, (mut tilemaps, players, transforms): Self::SystemData) {
        for (player, transform) in (&players, &transforms).join() {
            for tilemap in (&mut tilemaps).join() {
                let dim = tilemap.dimensions().clone();
                let mut clone = tilemap.clone();
                let my_map = SanityMap(clone);
                let trans = transform.translation();
                let curr_tile = tilemap
                    .to_tile(&Vector3::new(trans.x, trans.y, 0.), None)
                    .unwrap();
                let fov = field_of_view_set(Point::new(curr_tile.x, curr_tile.y), 4, &my_map);
                for z in 0..2 {
                    for x in 0..dim.x {
                        for y in 0..dim.y {
                            let p = Point::new(x, y);
                            if let Some(tile) = tilemap.get_mut(&Point3::new(x, y, z)) {
                                let vis = fov.contains(&p);
                                tile.visible = vis;
                                if vis {
                                    tile.visited = true;
                                    if !tile.walkable {
                                        if let Some(tile) =
                                            tilemap.get_mut(&Point3::new(x, y - 1, z))
                                        {
                                            tile.visible = vis;
                                            tile.visited = true;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
