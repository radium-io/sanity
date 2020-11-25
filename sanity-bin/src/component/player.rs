use amethyst::ecs::{Component, Entity, HashMapStorage};

#[derive(Debug, Default)]
pub struct Player {
    pub weapon: Option<Entity>,
}

impl Component for Player {
    type Storage = HashMapStorage<Self>;
}
