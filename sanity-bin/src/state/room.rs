use std::fmt::Debug;

use amethyst::{
    assets::{AssetStorage, Handle, Loader, ProgressCounter, RonFormat},
    core::{math::Point3, math::Vector3, Named, Transform},
    ecs::Join,
    input::{is_close_requested, is_key_down},
    prelude::*,
    renderer::{
        camera::Camera,
        formats::texture::ImageFormat,
        sprite::{SpriteSheet, SpriteSheetFormat},
        Texture,
    },
    tiles::{MapStorage, TileMap},
    ui::UiCreator,
    window::ScreenDimensions,
    winit,
};
use amethyst_utils::ortho_camera::{CameraNormalizeMode, CameraOrtho, CameraOrthoWorldCoordinates};
use direction::CardinalDirectionTable;
use rand::prelude::*;
use sanity_lib::tile::RoomTile;

#[derive(Debug, Default)]
pub struct RoomState {
    progress_counter: ProgressCounter,
    map_generation: usize,
}

fn load_sprite_sheet(world: &World, png_path: &str, ron_path: &str) -> Handle<SpriteSheet> {
    let loader = world.read_resource::<Loader>();
    let texture_handle = loader.load(
        png_path,
        ImageFormat::default(),
        (),
        &world.read_resource::<AssetStorage<Texture>>(),
    );
    loader.load(
        ron_path,
        SpriteSheetFormat(texture_handle),
        (),
        &world.read_resource::<AssetStorage<SpriteSheet>>(),
    )
}

use wfc::*;

struct ForbidCorner {
    width: i32,
    height: i32,
}
impl ForbidPattern for ForbidCorner {
    fn forbid<W: Wrap, R: Rng>(&mut self, fi: &mut ForbidInterface<W>, rng: &mut R) {
        fi.forbid_all_patterns_except(Coord::new(0, 0), 3, rng);
        fi.forbid_all_patterns_except(Coord::new(self.width - 1, 0), 5, rng);
        fi.forbid_all_patterns_except(Coord::new(0, self.height - 1), 51, rng);
        fi.forbid_all_patterns_except(Coord::new(self.width - 1, self.height - 1), 53, rng);
    }
}

fn gen_map(
    map: &mut TileMap<RoomTile>,
    pairs: &sanity_lib::assets::Pairs,
    width: u32,
    height: u32,
) {
    let mut rng = thread_rng();

    let mut v: Vec<PatternDescription> = Vec::new();

    let max_tiles = 78;
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
            wt = std::num::NonZeroU32::new(3000);
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
            println!("{:?} has pattern", idx);
            v.push(PatternDescription::new(
                wt,
                direction::CardinalDirectionTable::new_array([n, e, s, w]),
            ));
        } else {
            println!("{:?} has no pattern", idx);
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

    let mut wfc_run = wfc::RunBorrow::new_wrap_forbid(
        &mut context,
        &mut wave,
        &mut stats,
        wfc::wrap::WrapNone,
        ForbidCorner {
            width: width as i32,
            height: height as i32,
        },
        &mut rng,
    );

    println!("Running collapse!");

    wfc_run.collapse_retrying(wfc::retry::Forever, &mut rng);

    wave.grid().map_ref_with_coord(|c, cell| {
        if let Some(mut tile) = map.get_mut(&Point3::new(c.x as u32, c.y as u32, 0)) {
            tile.sprite = Some(
                cell.chosen_pattern_id()
                    .expect(&format!("Chosen tile for coord {:?}.", cell)) as usize,
            )
        }
    });
}

use amethyst::ecs::prelude::*;

impl SimpleState for RoomState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let StateData { world, .. } = data;

        world.register::<Named>();
        world.register::<Handle<sanity_lib::assets::Pairs>>();

        let (width, height) = {
            let dim = world.read_resource::<ScreenDimensions>();
            (dim.width(), dim.height())
        };

        let mut ortho = CameraOrtho::normalized(CameraNormalizeMode::Contain);
        ortho.world_coordinates = CameraOrthoWorldCoordinates {
            left: -width / 2.,
            right: width / 2.,
            top: height / 2.,
            bottom: -height / 2.,
            ..Default::default()
        };
        world
            .create_entity()
            .with(Transform::from(Vector3::new(0., 0., 1.1)))
            .with(Camera::standard_2d(width, height))
            .with(ortho)
            .named("camera")
            .build();

        let spritesheet_handle =
            load_sprite_sheet(&world, "Dungeon_Tileset.png", "Dungeon_Tileset.ron");

        let map = TileMap::<RoomTile>::new(
            Vector3::new(16, 16, 1), // The dimensions of the map
            Vector3::new(32, 32, 1), // The dimensions of each tile
            Some(spritesheet_handle),
        );

        // load the tile pairs for this tileset
        let pairs = {
            let loader = world.read_resource::<Loader>();
            loader.load(
                "Dungeon_Tileset.pairs.ron",
                RonFormat,
                &mut self.progress_counter,
                &world.read_resource::<AssetStorage<sanity_lib::assets::Pairs>>(),
            )
        };

        let t = Transform::default();
        world
            .create_entity()
            .with(map)
            .with(pairs)
            .with(t)
            .named("map")
            .build();

        // FIXME: move to global state?
        world.exec(|mut creator: UiCreator<'_>| {
            creator.create("ui/fps.ron", ());
        });
    }

    fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        if self.progress_counter.is_complete() && self.map_generation < 1 {
            data.world.exec(
                |(mut maps, pairs, assets): (
                    WriteStorage<'_, TileMap<RoomTile>>,
                    ReadStorage<'_, sanity_lib::assets::PairsHandle>,
                    Read<'_, AssetStorage<sanity_lib::assets::Pairs>>,
                )| {
                    for (map, pair) in (&mut maps, &pairs).join() {
                        gen_map(map, assets.get(pair).unwrap(), 16, 16);
                    }
                },
            );

            self.map_generation = 1;

            Trans::None
        } else {
            Trans::None
        }
    }

    fn handle_event(
        &mut self,
        data: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        if let StateEvent::Window(event) = &event {
            if is_close_requested(&event) || is_key_down(&event, winit::VirtualKeyCode::Escape) {
                Trans::Quit
            } else if is_key_down(&event, winit::VirtualKeyCode::F) {
                data.world.exec(
                    |(mut maps, pairs, assets): (
                        WriteStorage<'_, TileMap<RoomTile>>,
                        ReadStorage<'_, sanity_lib::assets::PairsHandle>,
                        Read<'_, AssetStorage<sanity_lib::assets::Pairs>>,
                    )| {
                        for (map, pair) in (&mut maps, &pairs).join() {
                            gen_map(map, assets.get(pair).unwrap(), 16, 16);
                        }
                    },
                );
                Trans::None
            } else {
                Trans::None
            }
        } else {
            Trans::None
        }
    }
}
