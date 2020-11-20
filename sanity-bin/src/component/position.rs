use amethyst::ecs::{Component, DenseVecStorage};
use bracket_pathfinding::prelude::Point;
use std::ops::Add;

#[derive(Clone)]
pub struct Position {
    pub pos: Point,
}

impl Component for Position {
    type Storage = DenseVecStorage<Self>;
}

impl Add<direction::CardinalDirection> for Position {
    type Output = Self;

    fn add(self, other: direction::CardinalDirection) -> Self {
        let c = other.coord();
        Self {
            pos: Point::new(self.pos.x + c.x, self.pos.y + c.y),
        }
    }
}
