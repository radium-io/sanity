use amethyst::{
    core::timing::Time,
    core::{math::Point3, Hidden, Transform},
    derive::SystemDesc,
    ecs::{
        prelude::{Read, System, SystemData, WriteStorage},
        Entities, Entity, Join, ReadStorage,
    },
    tiles::{Map, MapStorage, TileMap},
};
use bracket_pathfinding::prelude::Point;
use bracket_pathfinding::prelude::*;
use core::time::Duration;
use rand::seq::SliceRandom;
use rand::thread_rng;
use sanity_lib::map::SanityMap;
use sanity_lib::tile::RoomTile;
use std::cmp::Ordering;

#[derive(Default, SystemDesc)]
pub struct AISystem {
    last_move: Duration,
}

impl<'a> System<'a> for AISystem {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, TileMap<RoomTile>>,
        WriteStorage<'a, crate::component::MovementIntent>,
        WriteStorage<'a, crate::component::Position>,
        ReadStorage<'a, crate::component::Enemy>,
        ReadStorage<'a, crate::component::Player>,
        ReadStorage<'a, crate::component::Health>,
        Read<'a, Time>,
    );

    fn run(
        &mut self,
        (entities, mut tilemaps, mut intents, mut positions, enemies, players, healths, time): Self::SystemData,
    ) {
        for (entity, enemy) in (&entities, &enemies).join() {
            if let Some(intent) = intents.get(entity) {
                if intent.step == 0 {
                    // stop all movement intents from last player action
                    intents.remove(entity);
                } else {
                    return;
                }
            }
        }

        if time.absolute_time() - self.last_move > Duration::from_millis(2000) {
            self.last_move = time.absolute_time();

            for tilemap in (&mut tilemaps).join() {
                let dim = *tilemap.dimensions();
                let (width, height) = (dim.x, dim.y);

                let my_map = sanity_lib::map::SanityMap(tilemap);

                for (player_entity, player, player_pos) in (&entities, &players, &positions).join()
                {
                    let player_idx =
                        my_map.point2d_to_index(Point::new(player_pos.pos.x, player_pos.pos.y));

                    let dijkstra = DijkstraMap::new(width, height, &[player_idx], &my_map, 1000.);

                    for (entity, enemy, position, health) in
                        (&entities, &enemies, &positions, &healths).join()
                    {
                        if healths.get(player_entity).is_some()
                            && healths.get(player_entity).unwrap().current > 0
                        {
                            let e_pos = my_map.point2d_to_index(position.pos);
                            if let Some(target) =
                                DijkstraMap::find_lowest_exit(&dijkstra, e_pos, &my_map)
                            {
                                let target_pos = my_map.index_to_point2d(target);
                                let dist = my_map.get_pathing_distance(player_idx, e_pos);
                                let coord_pt = target_pos - position.pos;
                                let player_coord_pt = player_pos.pos - position.pos;

                                println!("{:?} {:?}", position.pos, dist);
                                if dist > 1. {
                                    intents.insert(
                                        entity,
                                        crate::component::MovementIntent {
                                            dir: direction::CardinalDirection::from_unit_coord(
                                                direction::Coord::new(coord_pt.x, coord_pt.y),
                                            ),
                                            step: 5,
                                        },
                                    );
                                } else {
                                    println!("Attack!");

                                    intents.insert(
                                        entity,
                                        crate::component::MovementIntent {
                                            dir: direction::CardinalDirection::from_unit_coord(
                                                direction::Coord::new(
                                                    player_coord_pt.x,
                                                    player_coord_pt.y,
                                                ),
                                            ),
                                            step: 5,
                                        },
                                    );
                                }
                            }
                        } else {
                            intents.insert(
                                entity,
                                crate::component::MovementIntent {
                                    dir: rand::random(),
                                    step: 5,
                                },
                            );
                        }
                    }
                }
            }
        }
    }
}
