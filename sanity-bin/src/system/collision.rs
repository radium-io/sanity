use amethyst::{
    derive::SystemDesc,
    ecs::{
        prelude::{System, SystemData, WriteStorage},
        Entities, Join, ReadStorage,
    },
    tiles::TileMap,
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
            mut healths,
            mut floor_maps,
        ): Self::SystemData,
    ) {
        for (entity, collision) in (&entities, &collisions).join() {
            if let Some(with) = collision.with {
                // Enemey collided with something due to move or attack
                if let Some(enemy) = enemies.get(entity) {
                    if let Some(player) = players.get(with) {
                        if let Some(player_health) = healths.get_mut(with) {
                            if player_health.current > 0 {
                                player_health.current -= 10; // Melee attack damage
                            }
                        }
                    }
                }

                // Player collided with something due to action
                if let Some(player) = players.get(entity) {
                    if let Some(enemy) = enemies.get(with) {
                        if let Some(enemy_health) = healths.get_mut(with) {
                            if enemy_health.current > 0 {
                                enemy_health.current -= 10; // Melee attack damage
                            }
                        }
                    }
                }

                // Projectile collided with something
                if let Some(projectile) = projectiles.get(entity) {
                    if let Some(enemy) = enemies.get(with) {
                        if let Some(enemy_health) = healths.get_mut(with) {
                            enemy_health.current -= projectile.damage as i32;
                        }
                    }
                }
            }
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
