use amethyst::{
    animation::{
        get_animation_set, AnimationCommand, AnimationControlSet, AnimationSet, EndControl,
    },
    core::timing::Time,
    derive::SystemDesc,
    ecs::prelude::{
        Entities, Entity, Join, Read, ReadStorage, System, SystemData, Write, WriteStorage,
    },
    renderer::{SpriteRender, Transparent},
    ui::{UiFinder, UiText, UiTransform},
    utils::fps_counter::FpsCounter,
};

#[derive(Default, SystemDesc)]
pub struct LoseSystem;

impl<'a> System<'a> for LoseSystem {
    type SystemData = (
        ReadStorage<'a, crate::component::Player>,
        ReadStorage<'a, crate::component::Health>,
        Write<'a, crate::state::Sanity>,
    );

    fn run(&mut self, (players, healths, mut sanity_res): Self::SystemData) {
        for (player, health) in (&players, &healths).join() {
            if health.current <= 0 && !sanity_res.game_over {
                sanity_res.game_over = true;
            }
        }
    }
}
