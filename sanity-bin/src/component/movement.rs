use amethyst::ecs::{Component, DenseVecStorage};

pub struct MovementIntent {
    pub dir: direction::CardinalDirection,
}

impl Component for MovementIntent {
    type Storage = DenseVecStorage<Self>;
}
