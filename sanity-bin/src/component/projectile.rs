use amethyst::ecs::{Component, DenseVecStorage};

#[derive(Clone, Default)]
pub struct Projectile {
    pub damage: u32,
}

impl Projectile {
    pub fn new(damage: u32) -> Self {
        Self { damage }
    }
}

impl Component for Projectile {
    type Storage = DenseVecStorage<Self>;
}
