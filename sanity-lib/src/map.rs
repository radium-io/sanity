use crate::tile::RoomTile;
use amethyst::{
    core::math::Point3,
    tiles::{Map, MapStorage, TileMap},
};
use bracket_pathfinding::prelude::*;

pub struct SanityMap<'a>(pub &'a mut TileMap<RoomTile>);

impl<'a> SanityMap<'a> {
    fn valid_exit(&self, loc: Point, delta: Point) -> Option<usize> {
        let destination = loc + delta;
        if self.in_bounds(destination) {
            let idx = self.point2d_to_index(destination);

            match self
                .0
                .get(&Point3::new(destination.x as u32, destination.y as u32, 0))
            {
                Some(tile) if tile.walkable => Some(idx),
                _ => None,
            }
        } else {
            None
        }
    }
}

impl<'a> BaseMap for SanityMap<'a> {
    fn is_opaque(&self, idx: usize) -> bool {
        let p = self.index_to_point2d(idx);
        if let Some(tile) = self.0.get(&Point3::new(p.x as u32, p.y as u32, 0)) {
            !tile.walkable
        } else {
            true
        }
    }

    fn get_available_exits(&self, idx: usize) -> SmallVec<[(usize, f32); 10]> {
        let mut exits = SmallVec::new();
        let location = self.index_to_point2d(idx);

        if let Some(idx) = self.valid_exit(location, Point::new(-1, 0)) {
            exits.push((idx, 1.0))
        }
        if let Some(idx) = self.valid_exit(location, Point::new(1, 0)) {
            exits.push((idx, 1.0))
        }
        if let Some(idx) = self.valid_exit(location, Point::new(0, -1)) {
            exits.push((idx, 1.0))
        }
        if let Some(idx) = self.valid_exit(location, Point::new(0, 1)) {
            exits.push((idx, 1.0))
        }

        if let Some(idx) = self.valid_exit(location, Point::new(-1, -1)) {
            exits.push((idx, 1.4))
        }
        if let Some(idx) = self.valid_exit(location, Point::new(1, -1)) {
            exits.push((idx, 1.4))
        }
        if let Some(idx) = self.valid_exit(location, Point::new(-1, 1)) {
            exits.push((idx, 1.4))
        }
        if let Some(idx) = self.valid_exit(location, Point::new(1, 1)) {
            exits.push((idx, 1.4))
        }

        exits
    }

    fn get_pathing_distance(&self, idx1: usize, idx2: usize) -> f32 {
        DistanceAlg::Pythagoras.distance2d(self.index_to_point2d(idx1), self.index_to_point2d(idx2))
    }
}

impl<'a> Algorithm2D for SanityMap<'a> {
    fn dimensions(&self) -> Point {
        let dim = self.0.dimensions();
        Point::new(dim.x, dim.y)
    }
}
