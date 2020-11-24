use amethyst::{
    animation::{
        get_animation_set, AnimationCommand, AnimationControlSet, AnimationSet, EndControl,
    },
    core::{math::Point3, Hidden, Transform},
    derive::SystemDesc,
    ecs::{
        prelude::{System, SystemData, WriteStorage},
        Entities, Entity, Join, ReadStorage,
    },
    renderer::{palette, SpriteRender, Transparent},
    tiles::{Map, MapStorage, TileMap},
};

#[derive(Default, SystemDesc)]
pub struct IdleSystem {}

impl<'a> System<'a> for IdleSystem {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, AnimationSet<usize, SpriteRender>>,
        WriteStorage<'a, AnimationControlSet<usize, SpriteRender>>,
        ReadStorage<'a, crate::component::Health>,
    );

    fn run(&mut self, (entities, animation_sets, mut control_sets, mut healths): Self::SystemData) {
        for (entity, animation_set, health) in (&entities, &animation_sets, &healths).join() {
            if health.current > 0 {
                let control_set = get_animation_set(&mut control_sets, entity).unwrap();
                control_set.add_animation(
                    0,
                    &animation_set.get(&0).unwrap(),
                    EndControl::Stay,
                    1.0,
                    AnimationCommand::Start,
                );
            }
        }
    }
}
