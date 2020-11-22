use amethyst::{
    animation::{
        get_animation_set, AnimationCommand, AnimationControlSet, AnimationSet, EndControl,
    },
    derive::SystemDesc,
    ecs::{
        prelude::{System, SystemData, WriteStorage},
        Entities, Entity, Join, ReadStorage,
    },
    renderer::{SpriteRender, Transparent},
};

#[derive(Default, SystemDesc)]
pub struct CollisionSystem {}

impl<'a> System<'a> for CollisionSystem {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, crate::component::Collision>,
        ReadStorage<'a, crate::component::Projectile>,
        ReadStorage<'a, crate::component::Player>,
        ReadStorage<'a, crate::component::Enemy>,
        ReadStorage<'a, AnimationSet<usize, SpriteRender>>,
        WriteStorage<'a, AnimationControlSet<usize, SpriteRender>>,
        WriteStorage<'a, crate::component::Health>,
    );

    fn run(
        &mut self,
        (
            entities,
            mut collisions,
            projectiles,
            players,
            enemies,
            animation_sets,
            mut control_sets,
            mut healths,
        ): Self::SystemData,
    ) {
        for (entity, _) in (&entities, &players).join() {
            // TODO: play wall collision ugh noise
            collisions.remove(entity);
        }

        let mut killed: Vec<Entity> = vec![];
        for (entity, _, animation_set, _, health) in (
            &entities,
            &collisions,
            &animation_sets,
            &enemies,
            &mut healths,
        )
            .join()
        {
            health.current -= 10;
            if health.current <= 0 {
                let control_set = get_animation_set(&mut control_sets, entity).unwrap();
                control_set.abort(0);
                control_set.add_animation(
                    1,
                    &animation_set.get(&1).unwrap(),
                    EndControl::Stay,
                    4.0,
                    AnimationCommand::Start,
                );
                killed.push(entity);
            }
        }

        for e in killed {
            healths.remove(e);
        }

        for (entity, _, _) in (&entities, &collisions, &projectiles).join() {
            // when projectiles collide with something they are destroyed
            // this should happen after their effects resolve
            // TODO: some projectiles may be piercing
            entities.delete(entity);
        }
    }
}
