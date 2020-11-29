use amethyst::ecs::{Component, HashMapStorage};
use rand::prelude::*;
use std::convert::AsRef;
use strum_macros::AsRefStr;

#[derive(Debug, AsRefStr)]
#[strum(serialize_all = "snake_case")]
pub enum Items {
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

impl Default for Items {
    fn default() -> Self {
        Items::Battery
    }
}

#[derive(Default)]
pub struct Item {
    pub item: Items,
}

impl Item {
    fn sprite(&self) -> &str {
        self.item.as_ref()
    }
}

impl Component for Item {
    type Storage = HashMapStorage<Self>;
}
