use amethyst::{core::math::Point3, tiles::{MapStorage, TileMap}};
use bracket_pathfinding::prelude::*;
use direction::Coord;
use rand::{Rng, thread_rng};
use sanity_lib::{tile::RoomTile, map::SanityMap};
use wfc::{PatternDescription, PatternTable};

use wfc::*;

struct ForbidCorner {
    width: i32,
    height: i32,
    start: Coord,
}
impl ForbidPattern for ForbidCorner {
    fn forbid<W: Wrap, R: Rng>(&mut self, fi: &mut ForbidInterface<W>, rng: &mut R) {
        for x in 0..self.width {
            fi.forbid_all_patterns_except(Coord::new(x, 0), 17, rng)
                .unwrap();
            fi.forbid_all_patterns_except(Coord::new(x, self.height - 1), 17, rng)
                .unwrap();
        }

        for y in 0..self.height {
            fi.forbid_all_patterns_except(Coord::new(0, y), 17, rng)
                .unwrap();
            fi.forbid_all_patterns_except(Coord::new(self.width - 1, y), 17, rng)
                .unwrap();
        }

        // TODO: place entrances and exits and some path between them
        fi.forbid_all_patterns_except(self.start, 6, rng).unwrap();
    }
}



pub fn gen_map(
    map: &mut TileMap<RoomTile>,
    pairs: &sanity_lib::assets::Pairs,
    width: u32,
    height: u32,
    start: Coord,
) {
    let mut v: Vec<PatternDescription> = Vec::new();

    let max_tiles = 115;
    for idx in 0..max_tiles {
        let mut n: Vec<u32> = pairs
            .ns
            .clone()
            .into_iter()
            .filter(|p| p.1 == idx && p.0 < max_tiles)
            .map(|p| p.0 as u32)
            .collect();
        let mut s: Vec<u32> = pairs
            .ns
            .clone()
            .into_iter()
            .filter(|p| p.0 == idx && p.1 < max_tiles)
            .map(|p| p.1 as u32)
            .collect();

        let mut w: Vec<u32> = pairs
            .we
            .clone()
            .into_iter()
            .filter(|p| p.1 == idx && p.0 < max_tiles)
            .map(|p| p.0 as u32)
            .collect();

        let mut e: Vec<u32> = pairs
            .we
            .clone()
            .into_iter()
            .filter(|p| p.0 == idx && p.1 < max_tiles)
            .map(|p| p.1 as u32)
            .collect();

        let mut wt = std::num::NonZeroU32::new(50);

        if idx == 6 {
            // FIXME: floor weighting
            wt = std::num::NonZeroU32::new(100);
        }

        if (n.len() > 0 || s.len() > 0) && (w.len() == 0 || e.len() == 0) {
            w.push(idx as u32);
            e.push(idx as u32);
            wt = std::num::NonZeroU32::new(1);
        }

        if (n.len() == 0 || s.len() == 0) && (w.len() > 0 || e.len() > 0) {
            n.push(idx as u32);
            s.push(idx as u32);
            wt = std::num::NonZeroU32::new(1);
        }

        if s.len() > 0 || e.len() > 0 || n.len() > 0 || w.len() > 0 {
            v.push(PatternDescription::new(
                wt,
                direction::CardinalDirectionTable::new_array([n, e, s, w]),
            ));
        } else {
            v.push(PatternDescription::new(
                wt,
                direction::CardinalDirectionTable::new_array([
                    vec![idx as u32],
                    vec![idx as u32],
                    vec![idx as u32],
                    vec![idx as u32],
                ]),
            ))
        }
    }

    let patterns: PatternTable<PatternDescription> = PatternTable::from_vec(v);

    let mut context = wfc::Context::new();
    let mut wave = wfc::Wave::new(wfc::Size::try_new(width, height).unwrap());
    let mut stats = wfc::GlobalStats::new(patterns);

    let mut rng = thread_rng();

    let mut wfc_run = wfc::RunBorrow::new_wrap_forbid(
        &mut context,
        &mut wave,
        &mut stats,
        wfc::wrap::WrapNone,
        ForbidCorner {
            width: width as i32,
            height: height as i32,
            start,
        },
        &mut rng,
    );

    println!("Running collapse!");

    wfc_run.collapse_retrying(wfc::retry::Forever, &mut rng);

    wave.grid().map_ref_with_coord(|c, cell| {
        if let Some(mut tile) = map.get_mut(&Point3::new(c.x as u32, c.y as u32, 0)) {
            let s = Some(
                cell.chosen_pattern_id()
                    .expect(&format!("Chosen tile for coord {:?}.", cell)) as usize,
            );
            tile.sprite = s;
            if s == Some(6)
                || s == Some(36)
                || s == Some(97)
                || s == Some(98)
                || s == Some(0)
                || s == Some(1)
                || s == Some(2)
                || s == Some(81)
                || s == Some(82)
            {
                tile.walkable = true;
            } else {
                tile.walkable = false;
            }
            s
        } else {
            None
        }
    });

    let clone = map.clone();
    let my_map = SanityMap(&clone);
    let dijkstra = DijkstraMap::new(
        width,
        height,
        &[my_map.point2d_to_index(Point::new(start.x, start.y))],
        &my_map,
        1000.,
    );

    for x in 0..width {
        for y in 0..height {
            let p = Point::new(x, y);
            if let Some(tile) = map.get_mut(&Point3::new(x, y, 0)) {
                if tile.walkable {
                    if dijkstra.map[my_map.point2d_to_index(p)] == std::f32::MAX {
                        tile.sprite = Some(17);
                        tile.walkable = false;

                        // TODO: remove surrounding tiles as well
                    }
                }
            }
        }
    }
}