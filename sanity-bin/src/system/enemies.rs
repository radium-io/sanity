use std::cmp::Ordering;

use amethyst::{
    core::{math::Point3, Transform},
    derive::SystemDesc,
    ecs::{
        prelude::{System, SystemData, WriteStorage},
        Entities, Join, LazyUpdate, ReadStorage,
    },
    prelude::*,
    renderer::Transparent,
    shred::{Read, ReadExpect},
    tiles::{Map, MapStorage, TileMap},
};
use bracket_pathfinding::prelude::{Point, *};
use sanity_lib::{map::SanityMap, tile::RoomTile};

use crate::resource::Sprited;

#[derive(Default, SystemDesc)]
pub struct EnemySystem {
    total_enemies: usize,
}

impl<'a> System<'a> for EnemySystem {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, TileMap<RoomTile>>,
        ReadStorage<'a, crate::component::Player>,
        WriteStorage<'a, Transform>,
        Read<'a, LazyUpdate>,
        ReadExpect<'a, crate::resource::Enemies>,
        ReadStorage<'a, crate::component::Enemy>,
    );

    fn run(
        &mut self,
        (entities, tilemaps, players, transforms, lazy, enemies_res, enemies): Self::SystemData,
    ) {
        if self.total_enemies < 1 {
            for tilemap in (&tilemaps).join() {
                let clone = tilemap.clone();
                let my_map = SanityMap(clone);

                for (transform, _) in (&transforms, &players).join() {
                    let dijkstra = {
                        if let Ok(tile) =
                            tilemap.to_tile(&transform.translation().xy().to_homogeneous(), None)
                        {
                            let idx = my_map.point2d_to_index(Point::new(tile.x, tile.y));

                            let map = DijkstraMap::new(
                                tilemap.dimensions().x,
                                tilemap.dimensions().y,
                                &[idx],
                                &my_map,
                                1000.,
                            );

                            Some(map)
                        } else {
                            None
                        }
                    };

                    if let Some(dijkstra) = dijkstra {
                        println!("{:?}", dijkstra.map);
                        if let Some(furthest) = dijkstra
                            .map
                            .iter()
                            .map(|x| if x == &f32::MAX { &0. } else { x })
                            .enumerate()
                            .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(Ordering::Equal))
                        {
                            let p = my_map.index_to_point2d(furthest.0);
                            if let Some(tile) = tilemap.get(&Point3::new(
                                p.x as u32,
                                p.y as u32,
                                sanity_lib::map::MapLayer::Walls as u32,
                            )) {
                                if tile.walkable {
                                    // should just store dijkstras for every entity that can move
                                    let w = tilemap
                                        .to_world(&Point3::new(p.x as u32, p.y as u32, 0), None);
                                    let mut t = Transform::default();
                                    t.set_translation(w);
                                    t.move_forward(2.);
                                    t.move_up(8.);

                                    lazy.create_entity(&entities)
                                        .with(Transparent)
                                        .with(t)
                                        .with(crate::component::Enemy)
                                        .with(enemies_res.new_sprite())
                                        .build();
                                    self.total_enemies += 1;
                                    println!("Spawn at {:?}", p);
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
