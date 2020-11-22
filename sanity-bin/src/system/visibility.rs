use amethyst::renderer::palette;
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
use sanity_lib::{map::SanityMap, tile::FloorTile, tile::RoomTile};

#[derive(Default, SystemDesc)]
pub struct VisibilitySystem {}

impl<'a> System<'a> for VisibilitySystem {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, TileMap<RoomTile>>,
        WriteStorage<'a, TileMap<FloorTile>>,
        ReadStorage<'a, crate::component::Player>,
        WriteStorage<'a, Hidden>,
        ReadStorage<'a, crate::component::Enemy>,
        ReadStorage<'a, crate::component::Position>,
    );

    fn run(
        &mut self,
        (entities, mut wall_maps, mut floor_maps, players, mut hiddens, enemies, positions): Self::SystemData,
    ) {
        for floor in (&mut floor_maps).join() {
            for walls in (&mut wall_maps).join() {
                for (_, position) in (&players, &positions).join() {
                    let dim = *walls.dimensions();
                    let mut c = walls.clone();
                    let my_map = SanityMap(&mut c);
                    let fov = field_of_view_set(position.pos, 3, &my_map);

                    for x in 0..dim.x {
                        for y in 0..dim.y {
                            let vis = fov.contains(&Point::new(x, y));

                            if let Some(tile) = walls.get_mut(&Point3::new(x, y, 0)) {
                                tile.visible = vis;

                                if vis {
                                    tile.visited = true;
                                    if !tile.walkable {
                                        // FIXME: map looks weird unless we can see tile above top wall tile
                                        if let Some(tile) = walls.get_mut(&Point3::new(x, y - 1, 0))
                                        {
                                            tile.visible = vis;
                                            tile.visited = true;
                                        }

                                        if let Some(tile) = floor.get_mut(&Point3::new(x, y - 1, 0))
                                        {
                                            tile.visible = vis;
                                            tile.visited = true;
                                        }
                                    } else {
                                        tile.tint = Some(palette::Srgba::new(1., 1., 1., 0.1));
                                    }
                                } else if tile.walkable {
                                    tile.tint = None;
                                }
                            }

                            if let Some(tile) = floor.get_mut(&Point3::new(x, y, 0)) {
                                tile.visible = vis;
                                if vis {
                                    tile.visited = true;
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
