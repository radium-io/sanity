use amethyst::ecs::{Component, HashMapStorage};

#[derive(Clone, Default)]
pub struct Weapon {
    pub damage_range: (u32, u32),
    pub name: &str,
    pub ranged: bool,
}

impl Component for Projectile {
    type Storage = HashMapStorage<Self>;
}
