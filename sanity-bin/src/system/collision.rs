use amethyst::{
    derive::SystemDesc,
    ecs::{
        prelude::{System, SystemData, WriteStorage},
        Entities, Join, ReadStorage,
    },
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
    );

    fn run(&mut self, (entities, mut collisions, projectiles, players, enemies): Self::SystemData) {
        for (entity, _) in (&entities, &players).join() {
            // TODO: play wall collision ugh noise
            collisions.remove(entity);
        }

        for (entity, _, _) in (&entities, &collisions, &enemies).join() {
            // probably hit an enemy
            entities.delete(entity);
        }

        for (entity, _, _) in (&entities, &collisions, &projectiles).join() {
            // when projectiles collide with something they are destroyed
            // this should happen after their effects resolve
            // TODO: some projectiles may be piercing
            entities.delete(entity);
        }
    }
}
