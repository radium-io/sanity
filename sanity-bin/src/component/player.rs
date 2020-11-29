use crate::component::item::ItemType;
use amethyst::ecs::{Component, Entity, HashMapStorage};

#[derive(Debug, Default)]
pub struct Player {
    pub weapon: Option<Entity>,
    pub inventory: Vec<ItemType>,
}

impl Player {
    pub fn sight(&self) -> i32 {
        let mut base = 3;
        if self.inventory.contains(&ItemType::Flashlight) {
            base += 1;
        }
        base
    }
}

impl Component for Player {
    type Storage = HashMapStorage<Self>;
}
