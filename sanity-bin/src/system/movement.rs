use amethyst::{
    animation::{
        get_animation_set, AnimationCommand, AnimationControlSet, AnimationSet, EndControl,
    },
    core::{math::Point3, Hidden, Transform},
    derive::SystemDesc,
    ecs::prelude::*,
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
        WriteStorage<'a, crate::component::Player>,
        ReadStorage<'a, AnimationSet<usize, SpriteRender>>,
        WriteStorage<'a, AnimationControlSet<usize, SpriteRender>>,
        ReadStorage<'a, crate::component::Item>,
        ReadStorage<'a, crate::component::Exit>,
        Write<'a, crate::state::Sanity>,
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
            mut players,
            animation_sets,
            mut control_sets,
            items,
            exits,
            mut sanity_res,
        ): Self::SystemData,
    ) {
        if let Some(map_ent) = sanity_res.level.last().unwrap_or(&None) {
            if let Some(tilemap) = tilemaps.get(*map_ent) {
                let enemy_positions: Vec<_> =
                    (&entities, &positions, &enemies, &healths).join().collect();

                let mut intents_to_cancel: Vec<Entity> = vec![];

                for (player_entity, player_pos, _) in (&entities, &positions, &players).join() {
                    // Player wants to move.
                    if let Some(player_intent) = intents.get(player_entity) {
                        let c = player_intent.dir.coord();
                        let p = Point::new(c.x, c.y);
                        let target = player_pos.pos + p;

                        // Enemy is in place that player want's to move.  Melee attack.
                        if let Some(enemy) = enemy_positions
                            .iter()
                            .find(|x| x.1.pos == target && x.1.map == *map_ent)
                        {
                            // there's an enemy on this position
                            intents.remove(player_entity);
                            intents.remove(enemy.0);
                            collisions.insert(
                                player_entity,
                                crate::component::Collision {
                                    location: target,
                                    with: Some(enemy.0),
                                },
                            );
                        }
                    }

                    // test if enemy attacking or invalid move
                    for (entity, position, intent, _) in
                        (&entities, &positions, &mut intents, &enemies).join()
                    {
                        let c = intent.dir.coord();
                        let p = Point::new(c.x, c.y);
                        let target = position.pos + p;

                        if target == player_pos.pos {
                            // enemy attacks player
                            if let Some(animation_set) = animation_sets.get(entity) {
                                let control_set =
                                    get_animation_set(&mut control_sets, entity).unwrap();
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
                                    with: Some(player_entity),
                                },
                            );
                        } else if let Some(enemy) = enemy_positions
                            .iter()
                            .find(|x| x.1.pos == target && x.1.map == *map_ent)
                        {
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

                    if let Some(tile) =
                        tilemap.get(&Point3::new(target.x as u32, target.y as u32, 0))
                    {
                        if tile.walkable {
                            if let Some(animation_set) = animation_sets.get(entity) {
                                let control_set =
                                    get_animation_set(&mut control_sets, entity).unwrap();
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

                            if intent.step > 0 {
                                intent.step -= 1;
                            }

                            if intent.step == 0 {
                                position.pos = target;
                                if players.get(entity).is_some() {
                                    println!("Moved to {:?}", target);
                                }
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
                        if *p_pos == **c_pos {
                            println!("Colission");
                            hiddens.insert(p_ent, Hidden);
                            // inserts a collision on the entity occupying space projectile is in
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

                // collision with items
                for (player, p_position) in (&mut players, &positions).join() {
                    for (ent, item, i_position) in (&entities, &items, &positions).join() {
                        if p_position == i_position {
                            println!("Collected item {:?}", item.item);
                            player.inventory.push(item.item);
                            entities.delete(ent);
                        }
                    }

                    for (ent, exit, e_position) in (&entities, &exits, &positions).join() {
                        if p_position == e_position {
                            println!("Exit Found!");
                            sanity_res.level.push(None);
                            sanity_res.floor.push(None);
                        }
                    }
                }
            }
        }
    }
}
