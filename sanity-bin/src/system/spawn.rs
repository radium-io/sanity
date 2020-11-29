use crate::{
    component::Position,
    resource::{Animated, Sprited},
};
use amethyst::{
    core::{math::Point3, Hidden, Transform},
    derive::SystemDesc,
    ecs::{
        prelude::{System, SystemData, WriteStorage},
        Entities, Join, LazyUpdate, ReadStorage,
    },
    prelude::*,
    renderer::Transparent,
    shred::{Read, ReadExpect},
    tiles::{Map, TileMap},
};
use bracket_pathfinding::prelude::*;
use rand::{seq::SliceRandom, thread_rng};
use sanity_lib::{map::SanityMap, tile::RoomTile};
use std::cmp::Ordering;

#[derive(Default, SystemDesc)]
pub struct SpawnSystem {}

impl<'a> System<'a> for SpawnSystem {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, TileMap<RoomTile>>,
        ReadStorage<'a, crate::component::Player>,
        WriteStorage<'a, Transform>,
        Read<'a, LazyUpdate>,
        ReadExpect<'a, crate::resource::Enemies>,
        ReadStorage<'a, crate::component::Enemy>,
        ReadStorage<'a, crate::component::Position>,
        ReadStorage<'a, crate::component::Health>,
        ReadStorage<'a, crate::component::Item>,
        ReadExpect<'a, crate::resource::Items>,
    );

    fn run(
        &mut self,
        (
            entities,
            mut walls,
            players,
            transforms,
            lazy,
            enemies_res,
            enemies,
            positions,
            healths,
            items,
            items_res,
        ): Self::SystemData,
    ) {
        let max_enemies = 10;
        let max_items = 1;

        if (&enemies, &healths).join().count() < max_enemies || (&items).join().count() < max_items
        {
            for tilemap in (&mut walls).join() {
                let my_map = SanityMap(tilemap);

                for (position, _) in (&positions, &players).join() {
                    let idx = my_map.point2d_to_index(position.pos);

                    let dijkstra = DijkstraMap::new(
                        my_map.0.dimensions().x,
                        my_map.0.dimensions().y,
                        &[idx],
                        &my_map,
                        1000.,
                    );

                    let mut near_to_far = dijkstra
                        .map
                        .iter()
                        .map(|x| if x > &1000. { &0. } else { x })
                        .enumerate()
                        .collect::<Vec<(usize, &f32)>>();

                    near_to_far
                        .sort_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(Ordering::Equal));

                    let mut rng = thread_rng();

                    // TODO: valid locations are hardcoded to 8 squares away
                    //  I would like to specify percentile brackets of distance
                    if let Some(spawnable) = near_to_far.rsplit(|x| *x.1 < 8.).next() {
                        let mut current_enemies = (&enemies, &healths).join().count();

                        while spawnable.len() > max_enemies && current_enemies < max_enemies {
                            let pos = spawnable.choose(&mut rng).unwrap();
                            let p = my_map.index_to_point2d(pos.0);

                            if (&enemies, &positions, &healths)
                                .join()
                                .any(|x| x.1.pos == p)
                            {
                                println!("Enemy already at position, trying a new position.");
                                continue;
                            }

                            if let Some(tile) = my_map.get(p) {
                                if tile.walkable {
                                    // should just store dijkstras for every entity that can move
                                    let w = my_map
                                        .0
                                        .to_world(&Point3::new(p.x as u32, p.y as u32, 0), None);
                                    let mut t = Transform::default();
                                    t.set_translation(w);
                                    t.move_forward(2.);
                                    t.move_up(8.); // sprite offset

                                    lazy.create_entity(&entities)
                                        .with(crate::component::Enemy)
                                        .with(Transparent)
                                        .with(Hidden)
                                        .with(Position { pos: p })
                                        .with(crate::component::Health {
                                            max: 20,
                                            current: 20,
                                        })
                                        .with(t)
                                        .with(enemies_res.new_animated_sprite())
                                        .build();

                                    current_enemies += 1;
                                    println!("Spawn at {:?}", p);
                                }
                            }
                        }

                        let mut current_items = (&items).join().count();

                        while spawnable.len() > max_items && current_items < max_items {
                            let pos = spawnable.choose(&mut rng).unwrap();
                            let p = my_map.index_to_point2d(pos.0);

                            if (&items, &positions).join().any(|x| x.1.pos == p) {
                                println!("Item already at position, trying a new position.");
                                continue;
                            }

                            if let Some(tile) = my_map.get(p) {
                                if tile.walkable {
                                    // should just store dijkstras for every entity that can move
                                    let w = my_map
                                        .0
                                        .to_world(&Point3::new(p.x as u32, p.y as u32, 0), None);

                                    lazy.create_entity(&entities)
                                        .with(crate::component::Item {
                                            item: crate::component::item::ItemType::Flashlight,
                                        })
                                        .with(Hidden)
                                        .with(Position { pos: p })
                                        .with(Transform::from(w))
                                        .with(items_res.new_sprite(
                                            crate::component::item::ItemType::Flashlight,
                                        ))
                                        .build();

                                    current_items += 1;
                                    println!("Spawn item at {:?}", p);
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
