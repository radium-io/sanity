use amethyst::{
    core::{math::Point3, timing::Time, Transform},
    derive::SystemDesc,
    ecs::{
        prelude::{System, SystemData, WriteStorage},
        Entities, Join, ReadStorage,
    },
    input::{InputHandler, StringBindings},
    renderer::SpriteRender,
    shred::{Read, ReadExpect},
    tiles::{Map, MapStorage, TileMap},
};
use bracket_pathfinding::prelude::Point;
use core::time::Duration;
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
        ): Self::SystemData,
    ) {
        if time.absolute_time() - self.last_move > Duration::from_millis(100) {
            for tilemap in (&tilemaps).join() {
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
                                    let world_pos = tilemap.to_world(&target_pt, None);
                                    t.set_translation(world_pos);
                                    entities
                                        .build_entity()
                                        .with(t, &mut transforms)
                                        .with(
                                            crate::component::Projectile::new(10),
                                            &mut projectiles,
                                        )
                                        .with(bullet_res.new_sprite(), &mut sprites)
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
