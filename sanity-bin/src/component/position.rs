use amethyst::{
    core::math::{Point2, Point3},
    ecs::{Component, DenseVecStorage, Entity},
};
use bracket_pathfinding::prelude::Point;
use direction::Coord;
use std::ops::Add;

#[derive(Clone, PartialEq)]
pub struct Position {
    pub pos: Point,
    pub map: Entity,
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

    pub fn coord(&self) -> Coord {
        direction::Coord::new(self.pos.x as i32, self.pos.y as i32)
    }
}
impl Add<direction::CardinalDirection> for Position {
    type Output = Self;

    fn add(self, other: direction::CardinalDirection) -> Self {
        let c = other.coord();
        Self {
            pos: Point::new(self.pos.x + c.x, self.pos.y + c.y),
            map: self.map,
        }
    }
}
