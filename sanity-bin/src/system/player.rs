use amethyst::{
    core::{math::Point3, timing::Time, Transform},
    derive::SystemDesc,
    ecs::{
        prelude::{System, SystemData, WriteStorage},
        Entities, Join, ReadStorage,
    },
    input::{InputHandler, StringBindings},
    shred::Read,
    tiles::{Map, MapStorage, TileMap},
};
use bracket_pathfinding::prelude::Point;
use core::time::Duration;
use sanity_lib::tile::RoomTile;

#[derive(Default, SystemDesc)]
pub struct PlayerSystem {
    last_move: Duration,
}

impl<'a> System<'a> for PlayerSystem {
    type SystemData = (
        Read<'a, InputHandler<StringBindings>>,
        ReadStorage<'a, crate::component::Player>,
        WriteStorage<'a, crate::component::MovementIntent>,
        Read<'a, Time>,
        Entities<'a>,
    );

    fn run(&mut self, (input, players, mut intents, time, entities): Self::SystemData) {
        for (entity, _) in (&entities, &players).join() {
            intents.remove(entity);
        }

        if time.absolute_time() - self.last_move > Duration::from_millis(100) {
            for (entity, _) in (&entities, &players).join() {
                for dir in &[
                    ("up", direction::CardinalDirection::North),
                    ("down", direction::CardinalDirection::South),
                    ("left", direction::CardinalDirection::West),
                    ("right", direction::CardinalDirection::East),
                ] {
                    if input.action_is_down(dir.0).unwrap_or(false) {
                        println!("{:?}", dir.0);
                        self.last_move = time.absolute_time();
                        intents.insert(entity, crate::component::MovementIntent { dir: dir.1 });
                    }
                }
            }
        }
    }
}
