use amethyst::{
    core::timing::Time,
    derive::SystemDesc,
    ecs::{
        prelude::{System, SystemData, WriteStorage},
        Entities, Join, ReadStorage,
    },
    input::{InputHandler, StringBindings},
    shred::Read,
};
use core::time::Duration;

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
            // stop all movement intents from last player action
            intents.remove(entity);

            // check if player is attempting to move again
            if time.absolute_time() - self.last_move > Duration::from_millis(150) {
                for dir in &[
                    ("up", direction::CardinalDirection::North),
                    ("down", direction::CardinalDirection::South),
                    ("left", direction::CardinalDirection::West),
                    ("right", direction::CardinalDirection::East),
                ] {
                    if input.action_is_down(dir.0).unwrap_or(false) {
                        println!("{:?}", dir.0);
                        self.last_move = time.absolute_time();
                        intents.insert(entity, crate::component::MovementIntent { dir: dir.1 }).unwrap();
                    }
                }
            }
        }
    }
}
