use amethyst::ecs::{Component, DenseVecStorage};

#[derive(Debug)]
pub struct Health {
    pub max: u32,
    pub current: i32,
}

impl Component for Health {
    type Storage = DenseVecStorage<Self>;
}
