use amethyst::core::math::{Point2, Point3};
use amethyst::ecs::{Component, DenseVecStorage};
use bracket_pathfinding::prelude::Point;
use std::ops::Add;

#[derive(Clone)]
pub struct Position {
    pub pos: Point,
    pub step: usize,
}

impl Component for Position {
    type Storage = DenseVecStorage<Self>;
}
impl Position {
    pub fn xy(&self) -> Point2<u32> {
        Point2::new(self.pos.x as u32, self.pos.y as u32)
    }

    pub fn xyz(&self) -> Point3<u32> {
        Point3::new(self.pos.x as u32, self.pos.y as u32, 0)
    }
}
impl Add<direction::CardinalDirection> for Position {
    type Output = Self;

    fn add(self, other: direction::CardinalDirection) -> Self {
        let c = other.coord();
        Self {
            pos: Point::new(self.pos.x + c.x, self.pos.y + c.y),
            step: 0,
        }
    }
}
