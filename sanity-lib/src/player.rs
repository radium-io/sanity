use amethyst::{
    core::math::Point3,
    ecs::{Component, HashMapStorage, NullStorage},
};
use bracket_pathfinding::prelude::*;

#[derive(Debug)]
pub struct Player {
    pub pos: Point3<u32>,
}

impl Player {
    pub fn new(x: u32, y: u32) -> Self {
        Self {
            pos: Point3::new(x, y, 0),
        }
    }

    pub fn pos(&self) -> Point {
        Point::new(self.pos.x, self.pos.y)
    }
}

impl Component for Player {
    type Storage = HashMapStorage<Self>;
}
