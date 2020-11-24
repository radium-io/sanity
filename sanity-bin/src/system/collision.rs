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
use sanity_lib::tile::FloorTile;

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
        WriteStorage<'a, TileMap<FloorTile>>,
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
            mut floor_maps,
        ): Self::SystemData,
    ) {
        for (entity, _player, collision) in (&entities, &players, &collisions).join() {
            if collision.with.is_some() {
                if let Some(enemy_health) = healths.get(collision.with.unwrap()) {
                    if enemy_health.current > 0 {
                        if let Some(h) = healths.get_mut(entity) {
                            h.current -= 10; // TODO: depend upon monsters weapon
                        }
                    }
                }
            }
        }

        let mut killed: Vec<Entity> = vec![];
        for (entity, collision, animation_set, _enemy, health) in (
            &entities,
            &collisions,
            &animation_sets,
            &enemies,
            &mut healths,
        )
            .join()
        {
            if let Some(with) = collision.with {
                if let Some(proj) = projectiles.get(with) {
                    health.current -= proj.damage as i32;
                } else {
                    health.current -= 10;
                }
            }
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

        let mut collisions_to_remove = vec![];
        for (entity, collision, _) in (&entities, &collisions, &projectiles).join() {
            // when projectiles collide with something they are destroyed
            // this should happen after their effects resolve
            // TODO: some projectiles may be piercing
            entities.delete(entity);
            if let Some(ent) = collision.with {
                collisions_to_remove.push(ent); // mark collision as stale
            }
        }

        // FIXME: not sure if any collisions should persist between ticks
        for (entity, _) in (&entities, !&projectiles).join() {
            collisions_to_remove.push(entity);
        }

        for e in collisions_to_remove {
            collisions.remove(e);
        }
    }
}
