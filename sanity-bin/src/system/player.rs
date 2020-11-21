use amethyst::{
    animation::{
        get_animation_set, AnimationCommand, AnimationControlSet, AnimationSet, EndControl,
    },
    core::timing::Time,
    derive::SystemDesc,
    ecs::{
        prelude::{System, SystemData, WriteStorage},
        Entities, Join, ReadStorage,
    },
    input::{InputHandler, StringBindings},
    renderer::SpriteRender,
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
        ReadStorage<'a, AnimationSet<usize, SpriteRender>>,
        WriteStorage<'a, AnimationControlSet<usize, SpriteRender>>,
    );

    fn run(
        &mut self,
        (input, players, mut intents, time, entities, animation_sets, mut control_sets): Self::SystemData,
    ) {
        for (entity, _) in (&entities, &players).join() {
            for (entity, animation_set, _) in (&entities, &animation_sets, &players).join() {
                let control_set = get_animation_set(&mut control_sets, entity).unwrap();
                control_set.add_animation(
                    0,
                    &animation_set.get(&0).unwrap(),
                    EndControl::Loop(None),
                    1.0,
                    AnimationCommand::Start,
                );
            }
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
                        self.last_move = time.absolute_time();
                        intents
                            .insert(entity, crate::component::MovementIntent { dir: dir.1 })
                            .unwrap();
                    }
                }
            }
        }
    }
}
