use amethyst::{
    core::math::Point3,
    ecs::Entities,
    shred::World,
    tiles::{MapStorage, TileMap},
};
use bracket_pathfinding::prelude::*;
use direction::Coord;
use rand::{random, thread_rng, Rng};
use sanity_lib::{map::SanityMap, tile::RoomTile};
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

fn to_vec(p: &Vec<(usize, usize)>, idx: usize, max: usize) -> (Vec<u32>, Vec<u32>) {
    (
        p.clone()
            .into_iter()
            .filter(|p| p.1 == idx && p.0 < max)
            .map(|p| p.0 as u32)
            .collect(),
        p.clone()
            .into_iter()
            .filter(|p| p.0 == idx && p.1 < max)
            .map(|p| p.1 as u32)
            .collect(),
    )
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
        let (mut n, mut s) = to_vec(&pairs.ns, idx, max_tiles);
        let (mut w, mut e) = to_vec(&pairs.we, idx, max_tiles);

        let mut wt = std::num::NonZeroU32::new(50);

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
        if let Some(mut tile) = map.get_mut(&Point3::new(
            c.x as u32,
            c.y as u32,
            sanity_lib::map::MapLayer::Walls as u32,
        )) {
            let s = Some(
                cell.chosen_pattern_id()
                    .expect(&format!("Chosen tile for coord {:?}.", cell)) as usize,
            );
            tile.sprite = s;
            tile.walkable = pairs.walkable.contains(&s.unwrap());
        }
    });

    let my_map = SanityMap(map);
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
            if dijkstra.map[my_map.point2d_to_index(p)] == std::f32::MAX {
                if let Some(tile) =
                    my_map
                        .0
                        .get_mut(&Point3::new(x, y, sanity_lib::map::MapLayer::Walls as u32))
                {
                    if tile.walkable {
                        println!("Removing unreachable {:?}", p);
                        tile.sprite = Some(pairs.null);
                        tile.walkable = false;

                        // TODO: remove surrounding tiles as well
                    }
                }
            }

            if let Some(floor) =
                my_map
                    .0
                    .get_mut(&Point3::new(x, y, sanity_lib::map::MapLayer::Floor as u32))
            {
                floor.sprite = Some(88);
            }
        }
    }
}
