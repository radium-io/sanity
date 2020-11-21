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
        ReadStorage<'a, Named>,
        WriteStorage<'a, Transform>,
        ReadStorage<'a, crate::component::MovementIntent>,
        WriteStorage<'a, crate::component::Collision>,
        WriteStorage<'a, crate::component::Position>,
        ReadStorage<'a, crate::component::Projectile>,
    );

    fn run(
        &mut self,
        (
            entities,
            tilemaps,
            names,
            mut transforms,
            intents,
            mut collisions,
            mut positions,
            projectiles,
        ): Self::SystemData,
    ) {
        for (tilemap, name) in (&tilemaps, &names).join() {
            if name.name == "walls" {
                for (entity, position, intent, transform) in
                    (&entities, &mut positions, &intents, &mut transforms).join()
                {
                    let c = intent.dir.coord();
                    let p = Point::new(c.x, c.y);
                    let target = position.pos + p;

                    if let Some(tile) =
                        tilemap.get(&Point3::new(target.x as u32, target.y as u32, 0))
                    {
                        if tile.walkable {
                            position.pos = target;
                            transform.prepend_translation_x(
                                c.x as f32 * tilemap.tile_dimensions().x as f32,
                            );
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

        let proj_pos = (&entities, &projectiles, &positions).join();

        let ent_pos: Vec<(Entity, &crate::component::Position, _)> =
            (&entities, &positions, !&projectiles).join().collect();

        for (p_ent, _, p_pos) in proj_pos {
            for (c_ent, c_pos, _) in ent_pos.iter() {
                if p_pos.pos == c_pos.pos {
                    println!("Collission");
                    // inserts a collision on the entity occupying space projectile is in
                    collisions.insert(
                        *c_ent,
                        crate::component::Collision {
                            location: p_pos.pos.clone(),
                            with: Some(p_ent),
                        },
                    );
                }
            }
        }
    }
}
