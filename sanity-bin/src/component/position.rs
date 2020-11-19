use amethyst::ecs::{Component, DenseVecStorage};
use bracket_pathfinding::prelude::Point;

#[derive(Clone)]
pub struct Position {
    pub pos: Point,
}

impl Component for Position {
    type Storage = DenseVecStorage<Self>;
}
