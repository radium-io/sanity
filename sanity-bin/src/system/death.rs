use amethyst::{
    animation::{
        get_animation_set, AnimationCommand, AnimationControlSet, AnimationSet, DeferStartRelation,
        EndControl,
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
pub struct DeathSystem {}

impl<'a> System<'a> for DeathSystem {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, AnimationSet<usize, SpriteRender>>,
        WriteStorage<'a, AnimationControlSet<usize, SpriteRender>>,
        WriteStorage<'a, crate::component::Health>,
    );

    fn run(&mut self, (entities, animation_sets, mut control_sets, mut healths): Self::SystemData) {
        let mut killed: Vec<Entity> = vec![];

        for (entity, animation_set, health) in (&entities, &animation_sets, &mut healths).join() {
            if health.current <= 0 {
                let control_set = get_animation_set(&mut control_sets, entity).unwrap();
                control_set.abort(0);
                control_set.abort(3);
                control_set.add_deferred_animation(
                    1,
                    &animation_set.get(&1).unwrap(),
                    EndControl::Stay,
                    4.0,
                    AnimationCommand::Start,
                    2,
                    DeferStartRelation::Start(1.),
                );

                killed.push(entity);
            }
        }

        for e in killed {
            healths.remove(e);
        }
    }
}
