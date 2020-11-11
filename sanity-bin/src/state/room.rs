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

fn gen_map(
    map: &mut TileMap<RoomTile>,
    pairs: &sanity_lib::assets::Pairs,
    width: u32,
    height: u32,
) {
    let mut rng = thread_rng();

    let mut v: Vec<PatternDescription> = Vec::new();
    for _ in 0..38 {
        let p = PatternDescription::new(
            std::num::NonZeroU32::new(1),
            direction::CardinalDirectionTable::default(),
        );
        v.push(p);
    }

    for p in pairs.ns.clone() {
        let first = v.get_mut(p.0).unwrap();
        first
            .allowed_neighbours
            .get_mut(direction::CardinalDirection::South)
            .push(p.1 as u32);

        let second = v.get_mut(p.1).unwrap();
        second
            .allowed_neighbours
            .get_mut(direction::CardinalDirection::North)
            .push(p.0 as u32);
    }
    for p in pairs.we.clone() {
        let first = v.get_mut(p.0).unwrap();
        first
            .allowed_neighbours
            .get_mut(direction::CardinalDirection::East)
            .push(p.1 as u32);

        let second = v.get_mut(p.1).unwrap();
        second
            .allowed_neighbours
            .get_mut(direction::CardinalDirection::West)
            .push(p.0 as u32);
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
        wfc::ForbidNothing,
        &mut rng,
    );

    println!("Running collapse!");

    wfc_run.collapse_retrying(wfc::retry::Forever, &mut rng);

    wave.grid().map_ref_with_coord(|c, cell| {
        let mut tile = map
            .get_mut(&Point3::new(c.x as u32, c.y as u32, 0))
            .expect(&format!("{:?}", c.x));
        tile.sprite = Some(cell.chosen_pattern_id().expect("Chosen tile for coord.") as usize)
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
            left: -width / 3. / 2.,
            right: width / 3. / 2.,
            top: -height / 3. / 2.,
            bottom: height / 3. / 2.,
            ..Default::default()
        };
        world
            .create_entity()
            .with(Transform::from(Vector3::new(0., 0., 2.)))
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
