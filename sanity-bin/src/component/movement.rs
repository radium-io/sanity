use amethyst::ecs::{Component, DenseVecStorage};

pub struct MovementIntent {
    pub dir: direction::CardinalDirection,
    pub step: usize,
}

impl Component for MovementIntent {
    type Storage = DenseVecStorage<Self>;
}
