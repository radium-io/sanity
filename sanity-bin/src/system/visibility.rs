use amethyst::{
    core::{math::Point3, Hidden},
    derive::SystemDesc,
    ecs::{
        prelude::{System, SystemData, WriteStorage},
        Entities, Join, ReadStorage,
    },
    tiles::{Map, MapStorage, TileMap},
};
use bracket_pathfinding::prelude::{field_of_view_set, Point};
use sanity_lib::{map::SanityMap, tile::RoomTile};

#[derive(Default, SystemDesc)]
pub struct VisibilitySystem {}

impl<'a> System<'a> for VisibilitySystem {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, TileMap<RoomTile>>,
        ReadStorage<'a, crate::component::Player>,
        WriteStorage<'a, Hidden>,
        ReadStorage<'a, crate::component::Enemy>,
        ReadStorage<'a, crate::component::Position>,
    );

    fn run(
        &mut self,
        (entities, mut tilemaps, players, mut hiddens, enemies, positions): Self::SystemData,
    ) {
        for (_, position) in (&players, &positions).join() {
            for tilemap in (&mut tilemaps).join() {
                let dim = *tilemap.dimensions();
                let my_map = SanityMap(tilemap);
                let fov = field_of_view_set(position.pos, 4, &my_map);

                for z in 0..2 {
                    for x in 0..dim.x {
                        for y in 0..dim.y {
                            let vis = fov.contains(&Point::new(x, y));

                            if let Some(tile) = tilemap.get_mut(&Point3::new(x, y, z)) {
                                tile.visible = vis;

                                if vis {
                                    tile.visited = true;
                                    if !tile.walkable {
                                        // FIXME: map looks weird unless we can see tile above top wall tile
                                        if let Some(tile) =
                                            tilemap.get_mut(&Point3::new(x, y - 1, z))
                                        {
                                            tile.visible = vis;
                                            tile.visited = true;
                                        }
                                    }
                                }
                            }

                            for (entity, _, position) in (&entities, &enemies, &positions).join() {
                                if x == position.pos.x as u32 && y == position.pos.y as u32 {
                                    if vis {
                                        hiddens.remove(entity);
                                    } else {
                                        hiddens.insert(entity, Hidden);
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
