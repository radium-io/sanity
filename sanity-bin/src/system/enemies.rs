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
use sanity_lib::{map::SanityMap, tile::RoomTile};
use std::cmp::Ordering;

#[derive(Default, SystemDesc)]
pub struct EnemySystem {
    total_enemies: usize,
}

impl<'a> System<'a> for EnemySystem {
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
    );

    fn run(
        &mut self,
        (
            entities,
            mut tilemaps,
            players,
            transforms,
            lazy,
            enemies_res,
            enemies,
            animation_sets,
            mut control_sets,
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

        let enemies = (&enemies).join().count();

        if enemies < 1 {
            for tilemap in (&mut tilemaps).join() {
                let my_map = SanityMap(tilemap);

                for (transform, _) in (&transforms, &players).join() {
                    let dijkstra = {
                        if let Ok(tile) = my_map
                            .0
                            .to_tile(&transform.translation().xy().to_homogeneous(), None)
                        {
                            let idx = my_map.point2d_to_index(Point::new(tile.x, tile.y));

                            let map = DijkstraMap::new(
                                my_map.0.dimensions().x,
                                my_map.0.dimensions().y,
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
                        if let Some(furthest) = dijkstra
                            .map
                            .iter()
                            .map(|x| if x > &1000. { &0. } else { x })
                            .enumerate()
                            .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(Ordering::Equal))
                        {
                            let p = my_map.index_to_point2d(furthest.0);
                            if let Some(tile) =
                                my_map.0.get(&Point3::new(p.x as u32, p.y as u32, 0))
                            {
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
