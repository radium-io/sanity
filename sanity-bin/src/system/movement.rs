use amethyst::{
    core::{math::Point3, timing::Time, Transform},
    derive::SystemDesc,
    ecs::{
        prelude::{System, SystemData, WriteStorage},
        Join, ReadStorage,
    },
    input::{InputHandler, StringBindings},
    shred::Read,
    tiles::{Map, MapStorage, TileMap},
};
use bracket_pathfinding::prelude::Point;
use core::time::Duration;
use sanity_lib::tile::RoomTile;

#[derive(Default, SystemDesc)]
pub struct MovementSystem {
    last_move: Duration,
}

impl<'a> System<'a> for MovementSystem {
    type SystemData = (
        ReadStorage<'a, TileMap<RoomTile>>,
        WriteStorage<'a, Transform>,
        Read<'a, InputHandler<StringBindings>>,
        WriteStorage<'a, crate::component::Player>,
        Read<'a, Time>,
    );

    fn run(&mut self, (tilemaps, mut transforms, input, mut players, time): Self::SystemData) {
        if time.absolute_time() - self.last_move < Duration::from_millis(100) {
        } else {
            for tilemap in (&tilemaps).join() {
                for (player, transform) in (&mut players, &mut transforms).join() {
                    if input.action_is_down("up").unwrap_or(false) {
                        self.last_move = time.absolute_time();
                        let above = player.pos() - Point::new(0, 1);
                        if let Some(tile) =
                            tilemap.get(&Point3::new(above.x as u32, above.y as u32, 0))
                        {
                            if tile.walkable {
                                transform.move_up(tilemap.tile_dimensions().y as f32);
                                player.pos = Point3::new(player.pos.x, player.pos.y - 1, 0);
                            }
                        }
                    }

                    if input.action_is_down("down").unwrap_or(false) {
                        self.last_move = time.absolute_time();
                        let below = player.pos() + Point::new(0, 1);
                        if let Some(tile) =
                            tilemap.get(&Point3::new(below.x as u32, below.y as u32, 0))
                        {
                            if tile.walkable {
                                transform.move_down(tilemap.tile_dimensions().y as f32);
                                player.pos = Point3::new(player.pos.x, player.pos.y + 1, 0);
                            }
                        }
                    }

                    if input.action_is_down("left").unwrap_or(false) {
                        self.last_move = time.absolute_time();
                        let above = player.pos() - Point::new(1, 0);
                        if let Some(tile) =
                            tilemap.get(&Point3::new(above.x as u32, above.y as u32, 0))
                        {
                            if tile.walkable {
                                transform.move_left(tilemap.tile_dimensions().x as f32);
                                player.pos = Point3::new(player.pos.x - 1, player.pos.y, 0);
                            }
                        }
                    }

                    if input.action_is_down("right").unwrap_or(false) {
                        self.last_move = time.absolute_time();
                        let below = player.pos() + Point::new(1, 0);
                        if let Some(tile) =
                            tilemap.get(&Point3::new(below.x as u32, below.y as u32, 0))
                        {
                            if tile.walkable {
                                transform.move_right(tilemap.tile_dimensions().x as f32);
                                player.pos = Point3::new(player.pos.x + 1, player.pos.y, 0);
                            }
                        }
                    }
                }
            }
        }
    }
}
