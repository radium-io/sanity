use amethyst::ecs::{Component, NullStorage};

#[derive(Default)]
pub struct Exit;

impl Component for Exit {
    type Storage = NullStorage<Self>;
}
