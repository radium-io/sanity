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

const BELOW: Point = Point::constant(0, 1);
const ABOVE: Point = Point::constant(0, -1);
const LEFT: Point = Point::constant(-1, 0);
const RIGHT: Point = Point::constant(1, 0);

impl<'a> System<'a> for MovementSystem {
    type SystemData = (
        ReadStorage<'a, TileMap<RoomTile>>,
        WriteStorage<'a, Transform>,
        Read<'a, InputHandler<StringBindings>>,
        WriteStorage<'a, crate::component::Player>,
        Read<'a, Time>,
    );

    fn run(&mut self, (tilemaps, mut transforms, input, mut players, time): Self::SystemData) {
        if time.absolute_time() - self.last_move > Duration::from_millis(100) {
            for tilemap in (&tilemaps).join() {
                for (player, transform) in (&mut players, &mut transforms).join() {
                    for dir in &[
                        ("up", ABOVE),
                        ("down", BELOW),
                        ("left", LEFT),
                        ("right", RIGHT),
                    ] {
                        if input.action_is_down(dir.0).unwrap_or(false) {
                            self.last_move = time.absolute_time();
                            println!("{:?}", player.pos());
                            let target = player.pos() + dir.1;
                            if let Some(tile) =
                                tilemap.get(&Point3::new(target.x as u32, target.y as u32, 0))
                            {
                                if tile.walkable {
                                    transform.prepend_translation_x(
                                        dir.1.x as f32 * tilemap.tile_dimensions().x as f32,
                                    );
                                    transform.prepend_translation_y(
                                        -dir.1.y as f32 * tilemap.tile_dimensions().y as f32,
                                        // note: world coords are inverted from grid coords on y
                                    );
                                    let new_pos = player.pos() + dir.1;
                                    player.pos = Point3::new(new_pos.x as u32, new_pos.y as u32, 0);
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
