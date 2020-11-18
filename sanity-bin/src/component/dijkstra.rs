use amethyst::ecs::{Component, HashMapStorage};
use bracket_pathfinding::prelude::DijkstraMap;

pub struct Dijkstra(pub DijkstraMap);

impl Component for Dijkstra {
    type Storage = HashMapStorage<Self>;
}
