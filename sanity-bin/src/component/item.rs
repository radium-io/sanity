use amethyst::ecs::{Component, HashMapStorage};
use rand::prelude::*;
use std::convert::AsRef;
use strum_macros::AsRefStr;

#[derive(Debug, PartialEq, Clone, Copy, AsRefStr)]
#[strum(serialize_all = "snake_case")]
pub enum ItemType {
    Battery,
    Binoculars,
    Book,
    Bottle,
    Bunsen,
    Camera,
    Double_Welder,
    Drill,
    Extinguiser,
    Flashlight,
    Game,
    Gasmask,
    Hacksaw,
    Hammer,
    IceCream,
    Knife,
    Lantern,
    Medkit,
    Microscope,
    Plant,
    Pliers,
    Pump,
    Radio,
    Sample,
    Screwdriver,
    Soylent,
    Spool,
    Syringe,
    Tablet,
    Thermos,
    Welder,
    Wrench,
}

impl Default for ItemType {
    fn default() -> Self {
        ItemType::Battery
    }
}

#[derive(Default)]
pub struct Item {
    pub item: ItemType,
}

impl Item {
    fn sprite(&self) -> &str {
        self.item.as_ref()
    }
}

impl Component for Item {
    type Storage = HashMapStorage<Self>;
}
