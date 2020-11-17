use amethyst::ecs::{Component, DenseVecStorage};
use bracket_pathfinding::prelude::Point;

pub struct Collision {
    pub location: Point
}

impl Component for Collision {
    type Storage = DenseVecStorage<Self>;
}