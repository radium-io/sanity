use amethyst::{
    core::{math::Point3, Hidden},
    derive::SystemDesc,
    ecs::{
        prelude::{System, SystemData, WriteStorage},
        Entities, Join, Read, ReadStorage,
    },
    renderer::palette,
    tiles::{Map, MapStorage, TileMap},
};
use bracket_pathfinding::prelude::{field_of_view_set, Point};
use sanity_lib::{
    map::SanityMap,
    tile::{FloorTile, RoomTile},
};

#[derive(Default, SystemDesc)]
pub struct VisibilitySystem {}

use std::collections::HashSet;

impl<'a> System<'a> for VisibilitySystem {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, TileMap<RoomTile>>,
        WriteStorage<'a, TileMap<FloorTile>>,
        ReadStorage<'a, crate::component::Player>,
        WriteStorage<'a, Hidden>,
        ReadStorage<'a, crate::component::Enemy>,
        ReadStorage<'a, crate::component::Position>,
        ReadStorage<'a, crate::component::Projectile>,
        Read<'a, crate::state::Sanity>,
    );

    fn run(
        &mut self,
        (
            entities,
            mut wall_maps,
            mut floor_maps,
            players,
            mut hiddens,
            enemies,
            positions,
            projectiles,
            sanity_res,
        ): Self::SystemData,
    ) {
        if let Some(f_ent) = sanity_res.floor.last().unwrap_or(&None) {
            if let Some(floor) = floor_maps.get_mut(*f_ent) {
                if let Some(map_ent) = sanity_res.level.last().unwrap_or(&None) {
                    if let Some(walls) = wall_maps.get_mut(*map_ent) {
                        for (player, position) in (&players, &positions).join() {
                            let dim = *walls.dimensions();
                            let mut c = walls.clone();
                            let my_map = SanityMap(&mut c);
                            let mut fov = field_of_view_set(position.pos, player.sight(), &my_map);

                            for (projectile, position) in (&projectiles, &positions).join() {
                                let f = field_of_view_set(position.pos, 1, &my_map);
                                fov.extend(&f);
                            }

                            for x in 0..dim.x {
                                for y in 0..dim.y {
                                    let vis = fov.contains(&Point::new(x, y));

                                    if let Some(tile) = walls.get_mut(&Point3::new(x, y, 0)) {
                                        tile.visible = vis;

                                        if vis {
                                            tile.visited = true;
                                            if !tile.walkable {
                                                // FIXME: map looks weird unless we can see tile above top wall tile
                                                if let Some(tile) =
                                                    walls.get_mut(&Point3::new(x, y - 1, 0))
                                                {
                                                    tile.visible = vis;
                                                    tile.visited = true;
                                                }

                                                if let Some(tile) =
                                                    floor.get_mut(&Point3::new(x, y - 1, 0))
                                                {
                                                    tile.visible = vis;
                                                    tile.visited = true;
                                                }
                                            } else {
                                                tile.tint =
                                                    Some(palette::Srgba::new(1., 1., 1., 0.1));
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

                                    for (entity, position) in (&entities, &positions).join() {
                                        if x == position.pos.x as u32 && y == position.pos.y as u32
                                        {
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
    }
}
