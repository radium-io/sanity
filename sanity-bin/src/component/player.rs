use amethyst::{
    core::math::Point3,
    ecs::{Component, NullStorage},
};
use bracket_pathfinding::prelude::*;

#[derive(Debug, Default)]
pub struct Player;

impl Component for Player {
    type Storage = NullStorage<Self>;
}
