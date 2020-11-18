use amethyst::ecs::{Component, DenseVecStorage, NullStorage};
use bracket_pathfinding::prelude::Point;

#[derive(Default)]
pub struct Enemy;

impl Component for Enemy {
    type Storage = NullStorage<Self>;
}
