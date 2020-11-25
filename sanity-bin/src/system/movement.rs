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
    renderer::SpriteRender,
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
        ReadStorage<'a, crate::component::Health>,
        WriteStorage<'a, Hidden>,
        ReadStorage<'a, crate::component::Player>,
        ReadStorage<'a, AnimationSet<usize, SpriteRender>>,
        WriteStorage<'a, AnimationControlSet<usize, SpriteRender>>,
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
            healths,
            mut hiddens,
            players,
            animation_sets,
            mut control_sets,
        ): Self::SystemData,
    ) {
        for tilemap in (&tilemaps).join() {
            let enemy_positions: Vec<_> =
                (&entities, &positions, &enemies, &healths).join().collect();

            // player walked in to enemy
            let mut intents_to_cancel: Vec<Entity> = vec![];
            for (entity, position, intent, _) in
                (&entities, &positions, &mut intents, &players).join()
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

            for (player_ent, player, player_pos, _) in
                (&entities, &players, &positions, &healths).join()
            {
                for (entity, position, intent, _) in
                    (&entities, &positions, &mut intents, &enemies).join()
                {
                    let c = intent.dir.coord();
                    let p = Point::new(c.x, c.y);
                    let target = position.pos + p;

                    if target == player_pos.pos {
                        // enemy attacks player
                        if let Some(animation_set) = animation_sets.get(entity) {
                            let control_set = get_animation_set(&mut control_sets, entity).unwrap();
                            if let Some(attack_anim) = animation_set.get(&2) {
                                control_set.add_animation(
                                    2,
                                    &attack_anim,
                                    EndControl::Stay,
                                    2.0,
                                    AnimationCommand::Start,
                                );
                            }
                        }

                        intents_to_cancel.push(entity);
                        collisions.insert(
                            entity,
                            crate::component::Collision {
                                location: target,
                                with: Some(player_ent),
                            },
                        );
                        collisions.insert(
                            player_ent,
                            crate::component::Collision {
                                location: target,
                                with: Some(entity),
                            },
                        );
                    } else if let Some(enemy) = enemy_positions.iter().find(|x| x.1.pos == target) {
                        // there's an enemy on this position
                        intents_to_cancel.push(entity);
                    }
                }
            }
            for ent in intents_to_cancel.iter() {
                intents.remove(*ent);
            }

            // move the enemy or player or projectile
            for (entity, position, intent, transform) in
                (&entities, &mut positions, &mut intents, &mut transforms).join()
            {
                let c = intent.dir.coord();
                let p = Point::new(c.x, c.y);
                let target = position.pos + p;

                if let Some(tile) = tilemap.get(&Point3::new(target.x as u32, target.y as u32, 0)) {
                    if tile.walkable {
                        if let Some(animation_set) = animation_sets.get(entity) {
                            let control_set = get_animation_set(&mut control_sets, entity).unwrap();
                            if let Some(jump_anim) = animation_set.get(&3) {
                                control_set.add_animation(
                                    3,
                                    &jump_anim,
                                    EndControl::Stay,
                                    2.0,
                                    AnimationCommand::Start,
                                );
                            }
                        }

                        transform.prepend_translation_x(
                            c.x as f32 * tilemap.tile_dimensions().x as f32 / 5.,
                        );
                        transform.prepend_translation_y(
                            -c.y as f32 * tilemap.tile_dimensions().y as f32 / 5.,
                            // note: world coords are inverted from grid coords on y
                        );

                        intent.step -= 1;

                        if intent.step == 0 {
                            position.pos = target;
                            if projectiles.get(entity).is_some() {
                                intent.step = 5;
                            }
                        }
                    } else {
                        // TODO: add a Collision component to the entity and resolve behavior in collision_system
                        collisions.insert(
                            entity,
                            crate::component::Collision {
                                location: target,
                                with: None,
                            },
                        );
                        intents_to_cancel.push(entity);
                    }
                }
            }

            for ent in intents_to_cancel.iter() {
                intents.remove(*ent);
            }

            let enemy_positions: Vec<_> =
                (&entities, &positions, &enemies, &healths).join().collect();

            // handle projectiles colliding with enemies
            for (p_ent, p_pos, _) in (&entities, &positions, &projectiles).join() {
                for (c_ent, c_pos, _, _) in enemy_positions.iter() {
                    if p_pos.pos == c_pos.pos {
                        println!("Colission");
                        hiddens.insert(p_ent, Hidden);
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

            for (entity, _) in (&entities, &projectiles).join() {
                hiddens.remove(entity);
            }
        }
    }
}
