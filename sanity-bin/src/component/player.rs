use amethyst::ecs::{Component, NullStorage};

#[derive(Debug, Default)]
pub struct Player;

impl Component for Player {
    type Storage = NullStorage<Self>;
}
