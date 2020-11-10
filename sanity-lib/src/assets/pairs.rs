use amethyst::{
    assets::{Asset, Handle},
    ecs::VecStorage,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Pairs {
    pub ns: Vec<(usize, usize)>,
    pub we: Vec<(usize, usize)>,
}

pub type PairsHandle = Handle<Pairs>;

impl Asset for Pairs {
    const NAME: &'static str = "crate::assets::Pairs";
    type Data = Self;
    type HandleStorage = VecStorage<PairsHandle>;
}
