use amethyst::{
    core::{math::Point3, Transform},
    derive::SystemDesc,
    ecs::{
        prelude::{System, SystemData, WriteStorage},
        Join, ReadStorage,
    },
    tiles::{Map, MapStorage, TileMap},
};
use bracket_pathfinding::prelude::Point;
use sanity_lib::tile::RoomTile;

#[derive(Default, SystemDesc)]
pub struct MovementSystem {}

impl<'a> System<'a> for MovementSystem {
    type SystemData = (
        ReadStorage<'a, TileMap<RoomTile>>,
        WriteStorage<'a, Transform>,
        WriteStorage<'a, crate::component::MovementIntent>,
    );

    fn run(&mut self, (tilemaps, mut transforms, mut intents): Self::SystemData) {
        for tilemap in (&tilemaps).join() {
            for (transform, intent) in (&mut transforms, &mut intents).join() {
                let c = intent.dir.coord();
                let p = Point::new(c.x, c.y);
                let curr_tile = tilemap
                    .to_tile(&transform.translation().xy().to_homogeneous(), None)
                    .unwrap();
                let target = Point::new(curr_tile.x, curr_tile.y) + p;

                if let Some(tile) = tilemap.get(&Point3::new(target.x as u32, target.y as u32, 0)) {
                    if tile.walkable {
                        transform
                            .prepend_translation_x(c.x as f32 * tilemap.tile_dimensions().x as f32);
                        transform.prepend_translation_y(
                            -c.y as f32 * tilemap.tile_dimensions().y as f32,
                            // note: world coords are inverted from grid coords on y
                        );
                    } else {
                        // TODO: add a Collision component to the entity and resolve behavior in collision_system
                    }
                }
            }
        }
    }
}
