use amethyst::{
    core::math::Point3,
    tiles::{Map, MapStorage, TileMap},
};
use bracket_pathfinding::prelude::*;
use direction::Coord;
use rand::{thread_rng, Rng};
use sanity_lib::{
    map::SanityMap,
    tile::{FloorTile, RoomTile},
};
use wfc::{PatternDescription, PatternTable, *};

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

fn to_vec(p: &[(usize, usize)], idx: usize, max: usize) -> (Vec<u32>, Vec<u32>) {
    (
        p.iter()
            .filter(|p| p.1 == idx && p.0 < max)
            .map(|p| p.0 as u32)
            .collect(),
        p.iter()
            .filter(|p| p.0 == idx && p.1 < max)
            .map(|p| p.1 as u32)
            .collect(),
    )
}

fn gen_patterns(pairs: &sanity_lib::assets::Pairs) -> Vec<PatternDescription> {
    let mut patterns: Vec<PatternDescription> = Vec::new();

    // this is the highest index we currently support in the dungeon spritesheet
    let max_tiles = 115;

    for idx in 0..max_tiles {
        let (mut n, mut s) = to_vec(&pairs.ns, idx, max_tiles);
        let (mut w, mut e) = to_vec(&pairs.we, idx, max_tiles);

        let mut wt = std::num::NonZeroU32::new(50);

        if (!n.is_empty() || !s.is_empty()) && (w.is_empty() || e.is_empty()) {
            w.push(idx as u32);
            e.push(idx as u32);
            wt = std::num::NonZeroU32::new(1);
        }

        if (n.is_empty() || s.is_empty()) && (!w.is_empty() || !e.is_empty()) {
            n.push(idx as u32);
            s.push(idx as u32);
            wt = std::num::NonZeroU32::new(1);
        }

        if !s.is_empty() || !e.is_empty() || !n.is_empty() || !w.is_empty() {
            patterns.push(PatternDescription::new(
                wt,
                direction::CardinalDirectionTable::new_array([n, e, s, w]),
            ));
        } else {
            // workaround for tiles with no matchers, we just state they can only match with self
            // limitation of wfc library is that every pattern index is considered
            patterns.push(PatternDescription::new(
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

    patterns
}

#[allow(clippy::many_single_char_names)]
pub fn gen_map(
    walls: &mut TileMap<RoomTile>,
    floor: &mut TileMap<FloorTile>,
    pairs: &sanity_lib::assets::Pairs,
    start: Coord,
) {
    let patterns = gen_patterns(&pairs);
    let mut context = wfc::Context::new();
    let (width, height) = (walls.dimensions().x, walls.dimensions().y);
    let stats = wfc::GlobalStats::new(PatternTable::from_vec(patterns));

    let mut rng = thread_rng(); // FIXME: allow seeding this

    let mut size = 0;

    while size as f32 / (width as f32 * height as f32) < 0.5 {
        println!("Generating new map");
        let mut wave = wfc::Wave::new(wfc::Size::try_new(width, height).unwrap());

        let mut wfc_run = wfc::RunBorrow::new_wrap_forbid(
            &mut context,
            &mut wave,
            &stats,
            wfc::wrap::WrapNone,
            ForbidCorner {
                width: width as i32,
                height: height as i32,
                start,
            },
            &mut rng,
        );

        wfc_run.collapse_retrying(wfc::retry::Forever, &mut rng);

        wave.grid().map_ref_with_coord(|c, cell| {
            if let Some(tile) = walls.get_mut(&Point3::new(c.x as u32, c.y as u32, 0)) {
                let sprite = cell.chosen_pattern_id().ok().map(|t| t as usize);

                *tile = RoomTile {
                    sprite,
                    walkable: pairs.walkable.contains(&sprite.unwrap()),
                    ..Default::default()
                };
            }
        });

        let my_map = SanityMap(walls);
        let player_idx = my_map.point2d_to_index(Point::new(start.x, start.y));
        let dijkstra = DijkstraMap::new(width, height, &[player_idx], &my_map, 1000.);

        for x in 0..width {
            for y in 0..height {
                let p = Point::new(x, y);
                if dijkstra.map[my_map.point2d_to_index(p)] == std::f32::MAX {
                    if let Some(tile) = my_map.0.get_mut(&Point3::new(x, y, 0)) {
                        if tile.walkable {
                            println!("Removing unreachable {:?}", p);
                            tile.sprite = Some(pairs.null);
                            tile.walkable = false;

                            // TODO: remove surrounding tiles as well
                        }
                    }
                } else {
                    // this tile is reachable
                    size += 1;
                }

                if let Some(floor_tile) = floor.get_mut(&Point3::new(x, y, 0)) {
                    floor_tile.visited = false;
                    floor_tile.visible = false;
                    floor_tile.tint = None;
                    floor_tile.sprite = Some(88);
                }
            }
        }

        println!(
            "{} walkable tiles, {}% walkable",
            size,
            size as f32 / (width * height) as f32 * 100.
        );
    }
}
