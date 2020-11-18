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
pub struct CollisionSystem {}

impl<'a> System<'a> for CollisionSystem {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, crate::component::Collision>,
        ReadStorage<'a, crate::component::Projectile>,
    );

    fn run(&mut self, (entities, collisions, projectiles): Self::SystemData) {
        for (entity, _, _) in (&entities, &collisions, &projectiles).join() {
            entities.delete(entity);
        }
    }
}
