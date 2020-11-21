use crate::{component::Position, resource::Animated};
use amethyst::{
    animation::{
        get_animation_set, AnimationCommand, AnimationControlSet, AnimationSet, EndControl,
    },
    core::{math::Point3, Hidden, Transform},
    derive::SystemDesc,
    ecs::{
        prelude::{System, SystemData, WriteStorage},
        Entities, Join, LazyUpdate, ReadStorage,
    },
    prelude::*,
    renderer::{SpriteRender, Transparent},
    shred::{Read, ReadExpect},
    tiles::{Map, MapStorage, TileMap},
};
use bracket_pathfinding::prelude::{Point, *};
use rand::seq::SliceRandom;
use rand::thread_rng;
use sanity_lib::{map::SanityMap, tile::RoomTile};
use std::cmp::Ordering;

#[derive(Default, SystemDesc)]
pub struct SpawnSystem {
    total_enemies: usize,
}

impl<'a> System<'a> for SpawnSystem {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, TileMap<RoomTile>>,
        ReadStorage<'a, crate::component::Player>,
        WriteStorage<'a, Transform>,
        Read<'a, LazyUpdate>,
        ReadExpect<'a, crate::resource::Enemies>,
        ReadStorage<'a, crate::component::Enemy>,
        ReadStorage<'a, AnimationSet<usize, SpriteRender>>,
        WriteStorage<'a, AnimationControlSet<usize, SpriteRender>>,
        ReadStorage<'a, crate::component::Position>,
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
            animation_sets,
            mut control_sets,
            positions,
        ): Self::SystemData,
    ) {
        for (entity, animation_set, _) in (&entities, &animation_sets, &enemies).join() {
            let control_set = get_animation_set(&mut control_sets, entity).unwrap();
            control_set.add_animation(
                0,
                &animation_set.get(&0).unwrap(),
                EndControl::Loop(None),
                1.0,
                AnimationCommand::Start,
            );
        }

        let mut enemies = (&enemies).join().count();

        if enemies < 1 {
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
                    if let Some(spawnable) = near_to_far.rsplit(|x| *x.1 < 2.).next() {
                        let positions = spawnable.choose_multiple(&mut rng, 4);

                        for pos in positions {
                            println!("{:?}", pos);
                            let p = my_map.index_to_point2d(pos.0);
                            if let Some(tile) = my_map.get(p) {
                                if tile.walkable {
                                    // should just store dijkstras for every entity that can move
                                    let w = my_map
                                        .0
                                        .to_world(&Point3::new(p.x as u32, p.y as u32, 0), None);
                                    let mut t = Transform::default();
                                    t.set_translation(w);
                                    t.move_forward(2.);
                                    t.move_up(8.);

                                    lazy.create_entity(&entities)
                                        .with(crate::component::Enemy)
                                        .with(Transparent)
                                        .with(Hidden)
                                        .with(Position { pos: p })
                                        .with(t)
                                        .with(enemies_res.new_animated_sprite())
                                        .build();

                                    enemies += 1;
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
