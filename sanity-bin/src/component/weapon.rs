use amethyst::ecs::{Component, HashMapStorage};
use rand::prelude::*;

#[derive(Clone, Default)]
pub struct Weapon {
    pub damage_range: (u32, u32),
    pub ranged: bool,
}

impl Component for Weapon {
    type Storage = HashMapStorage<Self>;
}

impl Weapon {
    pub fn fire(&self) -> super::projectile::Projectile {
        let mut rng = thread_rng();
        super::projectile::Projectile::new(rng.gen_range(self.damage_range.0, self.damage_range.1))
    }
}
