use amethyst::ecs::{Component, DenseVecStorage, Entity};
use bracket_pathfinding::prelude::Point;

pub struct Collision {
    pub location: Point,
    pub with: Option<Entity>,
}

impl Component for Collision {
    type Storage = DenseVecStorage<Self>;
}
