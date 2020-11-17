use amethyst::{
    core::math::Point3,
    ecs::{Component, DenseVecStorage, HashMapStorage},
};

pub struct MovementIntent {
    dir: direction::CardinalDirection,
}

impl Component for MovementIntent {
    type Storage = DenseVecStorage<Self>;
}
