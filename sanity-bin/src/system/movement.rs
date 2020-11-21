use amethyst::{
    core::{math::Point3, Named, Transform},
    derive::SystemDesc,
    ecs::{
        prelude::{System, SystemData, WriteStorage},
        Entities, Entity, Join, ReadStorage,
    },
    tiles::{Map, MapStorage, TileMap},
};
use bracket_pathfinding::prelude::Point;
use sanity_lib::tile::RoomTile;

#[derive(Default, SystemDesc)]
pub struct MovementSystem {}

impl<'a> System<'a> for MovementSystem {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, TileMap<RoomTile>>,
        WriteStorage<'a, Transform>,
        WriteStorage<'a, crate::component::MovementIntent>,
        WriteStorage<'a, crate::component::Collision>,
        WriteStorage<'a, crate::component::Position>,
        ReadStorage<'a, crate::component::Projectile>,
        ReadStorage<'a, crate::component::Enemy>,
    );

    fn run(
        &mut self,
        (
            entities,
            tilemaps,
            mut transforms,
            mut intents,
            mut collisions,
            mut positions,
            projectiles,
            enemies,
        ): Self::SystemData,
    ) {
        for tilemap in (&tilemaps).join() {
            // handle projectiles colliding with non-projectiles
            let enemy_positions: Vec<_> = (&entities, &positions, &enemies).join().collect();

            for (p_ent, p_pos, _) in (&entities, &positions, &projectiles).join() {
                for (c_ent, c_pos, _) in enemy_positions.iter() {
                    if p_pos.pos == c_pos.pos {
                        println!("Colission");
                        // inserts a collision on the entity occupying space projectile is in
                        collisions.insert(
                            *c_ent,
                            crate::component::Collision {
                                location: p_pos.pos,
                                with: Some(p_ent),
                            },
                        );
                        collisions.insert(
                            p_ent,
                            crate::component::Collision {
                                location: p_pos.pos,
                                with: Some(*c_ent),
                            },
                        );
                    }
                }
            }

            // walked in to enemy
            let mut intents_to_cancel: Vec<Entity> = vec![];
            for (entity, position, intent, _) in
                (&entities, &positions, &mut intents, !&projectiles).join()
            {
                let c = intent.dir.coord();
                let p = Point::new(c.x, c.y);
                let target = position.pos + p;

                if let Some(enemy) = enemy_positions.iter().find(|x| x.1.pos == target) {
                    // there's an enemy on this position
                    intents_to_cancel.push(entity);
                    collisions.insert(
                        entity,
                        crate::component::Collision {
                            location: target,
                            with: Some(enemy.0),
                        },
                    );
                    collisions.insert(
                        enemy.0,
                        crate::component::Collision {
                            location: target,
                            with: Some(entity),
                        },
                    );
                }
            }

            for ent in intents_to_cancel {
                intents.remove(ent);
            }

            // move the enemy or player
            for (entity, position, intent, transform) in
                (&entities, &mut positions, &intents, &mut transforms).join()
            {
                let c = intent.dir.coord();
                let p = Point::new(c.x, c.y);
                let target = position.pos + p;

                if let Some(tile) = tilemap.get(&Point3::new(target.x as u32, target.y as u32, 0)) {
                    if tile.walkable {
                        position.pos = target;
                        transform
                            .prepend_translation_x(c.x as f32 * tilemap.tile_dimensions().x as f32);
                        transform.prepend_translation_y(
                            -c.y as f32 * tilemap.tile_dimensions().y as f32,
                            // note: world coords are inverted from grid coords on y
                        );
                    } else {
                        // TODO: add a Collision component to the entity and resolve behavior in collision_system
                        collisions.insert(
                            entity,
                            crate::component::Collision {
                                location: target,
                                with: None,
                            },
                        );
                    }
                }
            }
        }
    }
}
