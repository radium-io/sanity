use amethyst::{
    core::{math::Point3, Transform},
    derive::SystemDesc,
    ecs::{
        prelude::{System, SystemData, WriteStorage},
        Entities, Join, ReadStorage,
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
        ReadStorage<'a, crate::component::MovementIntent>,
        WriteStorage<'a, crate::component::Collision>,
        WriteStorage<'a, crate::component::Position>,
    );

    fn run(
        &mut self,
        (entities, tilemaps, mut transforms, intents, mut collisions, mut positions): Self::SystemData,
    ) {
        for tilemap in (&tilemaps).join() {
            for (entity, position, intent, transform) in
                (&entities, &mut positions, &intents, &mut transforms).join()
            {
                let c = intent.dir.coord();
                let p = Point::new(c.x, c.y);
                let target = position.pos + p;

                if let Some(tile) = tilemap.get(&Point3::new(
                    target.x as u32,
                    target.y as u32,
                    sanity_lib::map::MapLayer::Walls as u32,
                )) {
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
                        collisions.insert(entity, crate::component::Collision { location: target });
                    }
                }
            }
        }
    }
}
