use amethyst::{
    derive::SystemDesc,
    ecs::prelude::{Entities, Join, ReadStorage, System, SystemData, Write},
};

#[derive(Default, SystemDesc)]
pub struct LoseSystem;

impl<'a> System<'a> for LoseSystem {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, crate::component::Player>,
        ReadStorage<'a, crate::component::Health>,
        Write<'a, crate::state::Sanity>,
    );

    fn run(&mut self, (entities, players, healths, mut sanity_res): Self::SystemData) {
        for (entity, player, health) in (&entities, &players, &healths).join() {
            if health.current <= 0 && !sanity_res.game_over {
                sanity_res.game_over = true;
            }
        }
    }
}
