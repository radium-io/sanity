use amethyst::ecs::{Component, DenseVecStorage};

pub struct Health {
    pub max: u32,
    pub current: u32,
}

impl Component for Health {
    type Storage = DenseVecStorage<Self>;
}
