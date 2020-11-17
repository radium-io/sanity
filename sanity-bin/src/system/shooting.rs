use amethyst::{
    core::{math::Point3, timing::Time, Hidden, Transform},
    derive::SystemDesc,
    ecs::{
        prelude::{System, SystemData, WriteStorage},
        Entities, Join, ReadStorage,
    },
    input::{InputHandler, StringBindings},
    renderer::{SpriteRender, Transparent},
    shred::{Read, ReadExpect},
    tiles::{Map, MapStorage, TileMap},
};
use bracket_pathfinding::prelude::Point;
use core::time::Duration;
use direction::Coord;
use sanity_lib::tile::RoomTile;

use crate::resource::Sprited;

#[derive(Default, SystemDesc)]
pub struct ShootingSystem {
    last_move: Duration,
}

const BELOW: Point = Point::constant(0, 1);
const ABOVE: Point = Point::constant(0, -1);
const LEFT: Point = Point::constant(-1, 0);
const RIGHT: Point = Point::constant(1, 0);

impl<'a> System<'a> for ShootingSystem {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, TileMap<RoomTile>>,
        WriteStorage<'a, Transform>,
        Read<'a, InputHandler<StringBindings>>,
        WriteStorage<'a, crate::component::Projectile>,
        ReadStorage<'a, crate::component::Player>,
        WriteStorage<'a, SpriteRender>,
        Read<'a, Time>,
        ReadExpect<'a, crate::resource::Bullets>,
        WriteStorage<'a, crate::component::MovementIntent>,
        WriteStorage<'a, Transparent>,
        WriteStorage<'a, Hidden>,
    );

    fn run(
        &mut self,
        (
            entities,
            tilemaps,
            mut transforms,
            input,
            mut projectiles,
            players,
            mut sprites,
            time,
            bullet_res,
            mut intents,
            mut transparents,
            mut hiddens,
        ): Self::SystemData,
    ) {
        for tilemap in (&tilemaps).join() {
            if time.absolute_time() - self.last_move > Duration::from_millis(300) {
                for player in (&players).join() {
                    for shoot_dir in &[
                        ("shoot_up", ABOVE),
                        ("shoot_down", BELOW),
                        ("shoot_left", LEFT),
                        ("shoot_right", RIGHT),
                    ] {
                        if input.action_is_down(shoot_dir.0).unwrap_or(false) {
                            self.last_move = time.absolute_time();
                            let spawn_pos = player.pos() + shoot_dir.1;
                            let target_pt = Point3::new(spawn_pos.x as u32, spawn_pos.y as u32, 0);
                            if let Some(tile) = tilemap.get(&target_pt) {
                                if tile.walkable {
                                    let mut t = Transform::default();
                                    let world_pos = tilemap.to_world(&player.pos, None);

                                    println!("{:?}", world_pos);
                                    t.set_translation(world_pos);
                                    entities
                                        .build_entity()
                                        .with(Transparent, &mut transparents)
                                        .with(t, &mut transforms)
                                        .with(
                                            crate::component::Projectile::new(10),
                                            &mut projectiles,
                                        )
                                        .with(
                                            crate::component::MovementIntent {
                                                dir: direction::CardinalDirection::from_unit_coord(
                                                    Coord::new(shoot_dir.1.x, shoot_dir.1.y),
                                                ),
                                            },
                                            &mut intents,
                                        )
                                        .with(bullet_res.new_sprite(), &mut sprites)
                                        .with(Hidden, &mut hiddens)
                                        .build();
                                }
                            }
                        }
                    }
                }
            }

            for (e, projectile, transform, intent) in
                (&entities, &mut projectiles, &mut transforms, &intents).join()
            {
                if let Ok(target) = tilemap.to_tile(transform.translation(), None) {
                    // move existing bullets until collision
                    match intent.dir {
                        direction::CardinalDirection::North => {
                            if let Some(tile) = tilemap.get(&Point3::new(target.x, target.y - 1, 0))
                            {
                                if tile.walkable {
                                    //  TODO: check for collidable things
                                    transform.move_up(tilemap.tile_dimensions().y as f32 / 3.);
                                    hiddens.remove(e);
                                } else {
                                    entities.delete(e);
                                }
                            }
                        }
                        direction::CardinalDirection::East => {
                            if let Some(tile) = tilemap.get(&Point3::new(target.x + 1, target.y, 0))
                            {
                                if tile.walkable {
                                    //  TODO: check for collidable things
                                    transform.move_right(tilemap.tile_dimensions().x as f32 / 3.);
                                    hiddens.remove(e);
                                } else {
                                    entities.delete(e);
                                }
                            }
                        }
                        direction::CardinalDirection::South => {
                            if let Some(tile) = tilemap.get(&Point3::new(target.x, target.y + 1, 0))
                            {
                                if tile.walkable {
                                    //  TODO: check for collidable things
                                    transform.move_down(tilemap.tile_dimensions().y as f32 / 3.);
                                    hiddens.remove(e);
                                } else {
                                    entities.delete(e);
                                }
                            }
                        }
                        direction::CardinalDirection::West => {
                            if let Some(tile) = tilemap.get(&Point3::new(target.x - 1, target.y, 0))
                            {
                                if tile.walkable {
                                    //  TODO: check for collidable things
                                    transform.move_left(tilemap.tile_dimensions().x as f32 / 3.);
                                    hiddens.remove(e);
                                } else {
                                    entities.delete(e);
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
