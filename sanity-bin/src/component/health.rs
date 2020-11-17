use amethyst::ecs::{Component, DenseVecStorage};

pub struct Health {
    max: u32,
    current: u32,
}

impl Component for Health {
    type Storage = DenseVecStorage<Self>;
}
