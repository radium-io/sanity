use amethyst::{
    core::{math::Point3, timing::Time, Transform},
    derive::SystemDesc,
    ecs::{
        prelude::{System, SystemData, WriteStorage},
        Entities, Join, LazyUpdate, ReadStorage,
    },
    input::{InputHandler, StringBindings},
    prelude::Builder,
    renderer::Transparent,
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
        ReadStorage<'a, Transform>,
        Read<'a, InputHandler<StringBindings>>,
        WriteStorage<'a, crate::component::Projectile>,
        ReadStorage<'a, crate::component::Player>,
        Read<'a, Time>,
        ReadExpect<'a, crate::resource::Bullets>,
        WriteStorage<'a, crate::component::MovementIntent>,
        Read<'a, LazyUpdate>,
        ReadStorage<'a, crate::component::Position>,
    );

    fn run(
        &mut self,
        (
            entities,
            tilemaps,
            transforms,
            input,
            mut projectiles,
            players,
            time,
            bullet_res,
            intents,
            lazy,
            positions,
        ): Self::SystemData,
    ) {
        for tilemap in (&tilemaps).join() {
            if time.absolute_time() - self.last_move > Duration::from_millis(300) {
                for (player, player_pos) in (&players, &positions).join() {
                    for shoot_dir in &[
                        ("shoot_up", ABOVE),
                        ("shoot_down", BELOW),
                        ("shoot_left", LEFT),
                        ("shoot_right", RIGHT),
                    ] {
                        if input.action_is_down(shoot_dir.0).unwrap_or(false) {
                            self.last_move = time.absolute_time();
                            let spawn_pos = player_pos.pos + shoot_dir.1;
                            let target_pt = Point3::new(
                                spawn_pos.x as u32,
                                spawn_pos.y as u32,
                                sanity_lib::map::MapLayer::Walls as u32,
                            );
                            if let Some(tile) = tilemap.get(&target_pt) {
                                if tile.walkable {
                                    let mut t = Transform::default();
                                    let world_pos = tilemap.to_world(
                                        &Point3::new(
                                            player_pos.pos.x as u32,
                                            player_pos.pos.y as u32,
                                            sanity_lib::map::MapLayer::Walls as u32,
                                        ),
                                        None,
                                    );
                                    t.set_translation(world_pos);

                                    lazy.create_entity(&entities)
                                        .with(Transparent)
                                        .with(t)
                                        .with(crate::component::Projectile::new(10))
                                        .with(player_pos.clone())
                                        .with(crate::component::MovementIntent {
                                            dir: direction::CardinalDirection::from_unit_coord(
                                                Coord::new(shoot_dir.1.x, shoot_dir.1.y),
                                            ),
                                        })
                                        .with(bullet_res.new_sprite())
                                        .build();
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
