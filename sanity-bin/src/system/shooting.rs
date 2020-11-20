use amethyst::{
    core::{math::Point3, timing::Time, Transform},
    derive::SystemDesc,
    ecs::{
        prelude::{System, SystemData},
        Entities, Join, LazyUpdate, ReadStorage,
    },
    input::{InputHandler, StringBindings},
    prelude::Builder,
    renderer::Transparent,
    shred::{Read, ReadExpect},
    tiles::{Map, MapStorage, TileMap},
};
use core::time::Duration;
use sanity_lib::tile::RoomTile;

use crate::resource::Sprited;

#[derive(Default, SystemDesc)]
pub struct ShootingSystem {
    last_move: Duration,
}

use direction::CardinalDirection::*;

impl<'a> System<'a> for ShootingSystem {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, TileMap<RoomTile>>,
        Read<'a, InputHandler<StringBindings>>,
        ReadStorage<'a, crate::component::Player>,
        Read<'a, Time>,
        ReadExpect<'a, crate::resource::Bullets>,
        Read<'a, LazyUpdate>,
        ReadStorage<'a, crate::component::Position>,
    );

    fn run(
        &mut self,
        (entities, tilemaps, input, players, time, bullet_res, lazy, positions): Self::SystemData,
    ) {
        for tilemap in (&tilemaps).join() {
            if time.absolute_time() - self.last_move > Duration::from_millis(300) {
                for (_, player_pos) in (&players, &positions).join() {
                    for shoot_dir in &[
                        ("shoot_up", North),
                        ("shoot_down", South),
                        ("shoot_left", West),
                        ("shoot_right", East),
                    ] {
                        if input.action_is_down(shoot_dir.0).unwrap_or(false) {
                            self.last_move = time.absolute_time();

                            let spawn_pos = player_pos.clone() + shoot_dir.1;
                            let target_pt = Point3::new(
                                spawn_pos.pos.x as u32,
                                spawn_pos.pos.y as u32,
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
                                        .with(crate::component::MovementIntent { dir: shoot_dir.1 })
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
