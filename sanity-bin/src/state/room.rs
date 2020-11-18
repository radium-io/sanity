use amethyst::{
    assets::{AssetStorage, Handle, Loader, ProgressCounter, RonFormat},
    core::{
        math::{Point2, Point3, Vector3},
        Named, Parent, Transform,
    },
    input::{is_close_requested, is_key_down},
    prelude::*,
    renderer::{camera::Camera, SpriteRender, Transparent},
    tiles::{MapStorage, TileMap},
    ui::UiCreator,
    utils::ortho_camera::{CameraNormalizeMode, CameraOrtho, CameraOrthoWorldCoordinates},
    window::ScreenDimensions,
    winit,
};
use direction::Coord;
use rand::prelude::*;
use sanity_lib::{map::SanityMap, tile::RoomTile};
use std::fmt::Debug;

#[derive(Debug, Default)]
pub struct RoomState {
    progress_counter: ProgressCounter,
    map_generation: usize,
    width: u32,
    height: u32,
}

impl RoomState {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            ..Default::default()
        }
    }
}

use amethyst::ecs::prelude::*;
use strum::EnumCount;

fn init_camera(world: &mut World, player: Entity) {
    let (width, height) = {
        let dim = world.read_resource::<ScreenDimensions>();
        (dim.width(), dim.height())
    };

    let mut ortho = CameraOrtho::normalized(CameraNormalizeMode::Contain);
    let std = Camera::standard_2d(width / 2., height / 2.);
    ortho.world_coordinates = CameraOrthoWorldCoordinates {
        left: -width / 2.,
        right: width / 2.,
        top: height / 2.,
        bottom: -height / 2.,
        near: 0.125,
        far: 2000.,
    };

    world
        .create_entity()
        .with(Transform::from(Vector3::new(0., 0., 1000.)))
        .with(std)
        //.with(ortho)
        .with(Parent { entity: player })
        .named("camera")
        .build();
}

fn init_map(width: u32, height: u32, world: &mut World, progress: &mut ProgressCounter) {
    let spritesheet_handle =
        crate::resource::load_sprite_sheet(&world, "Dungeon_Tileset.png", "Dungeon_Tileset.ron");

    let map = TileMap::<RoomTile>::new(
        Vector3::new(width, height, sanity_lib::map::MapLayer::COUNT as u32), // The dimensions of the map
        Vector3::new(32, 32, 1), // The dimensions of each tile
        Some(spritesheet_handle),
    );

    // load the tile pairs for this tileset
    let pairs = {
        let loader = world.read_resource::<Loader>();
        loader.load(
            "Dungeon_Tileset.pairs.ron",
            RonFormat,
            progress,
            &world.read_resource::<AssetStorage<sanity_lib::assets::Pairs>>(),
        )
    };

    let mut c_t = Transform::default();
    c_t.move_forward(5.);
    world
        .create_entity()
        .with(map)
        .with(pairs)
        .with(c_t)
        .build();
}

fn init_player(width: u32, height: u32, world: &mut World) -> Entity {
    let sprite_sheet = crate::resource::load_sprite_sheet(
        &world,
        "sprites/Space Cadet.png",
        "sprites/Space Cadet.ron",
    );
    let mut t = Transform::default();
    t.move_backward(10.);
    t.move_up(8.);
    world
        .create_entity()
        .with(SpriteRender::new(sprite_sheet.clone(), 0))
        .with(Transparent)
        .with(t)
        .with(crate::component::Player::new(width / 2, height / 2))
        .build()
}

impl SimpleState for RoomState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let StateData { world, .. } = data;

        // register components, may be able to remove if used by system
        world.register::<Named>();
        world.register::<Handle<sanity_lib::assets::Pairs>>();

        // insert resources in to world
        let sheet = crate::resource::load_sprite_sheet(
            &world,
            "sprites/bullets.png",
            "sprites/bullets.ron",
        );
        world.insert(crate::resource::Bullets { sheet });

        let player = init_player(self.width, self.height, world);
        init_camera(world, player);
        init_map(self.width, self.height, world, &mut self.progress_counter);

        // FIXME: move to global state?
        world.exec(|mut creator: UiCreator<'_>| {
            creator.create("ui/fps.ron", ());
        });
    }

    fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        if self.progress_counter.is_complete() && self.map_generation < 1 {
            data.world.exec(
                |(mut maps, pairs, assets, players): (
                    WriteStorage<'_, TileMap<RoomTile>>,
                    ReadStorage<'_, sanity_lib::assets::PairsHandle>,
                    Read<'_, AssetStorage<sanity_lib::assets::Pairs>>,
                    ReadStorage<'_, crate::component::Player>,
                )| {
                    for player in (&players).join() {
                        for (map, pair) in (&mut maps, &pairs).join() {
                            crate::map::gen_map(
                                map,
                                assets.get(pair).unwrap(),
                                self.width,
                                self.height,
                                Coord::new(player.pos.x as i32, player.pos.y as i32),
                            );
                        }
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
                    |(mut maps, pairs, assets, players): (
                        WriteStorage<'_, TileMap<RoomTile>>,
                        ReadStorage<'_, sanity_lib::assets::PairsHandle>,
                        Read<'_, AssetStorage<sanity_lib::assets::Pairs>>,
                        ReadStorage<'_, crate::component::Player>,
                    )| {
                        for player in (&players).join() {
                            if player.pos.xy() < Point2::new(self.width - 3, self.height - 3)
                                && player.pos.xy() > Point2::new(2, 2)
                            {
                                for (map, pair) in (&mut maps, &pairs).join() {
                                    crate::map::gen_map(
                                        map,
                                        assets.get(pair).unwrap(),
                                        self.width,
                                        self.height,
                                        Coord::new(player.pos.x as i32, player.pos.y as i32),
                                    );
                                }
                            } else {
                                println!("Player too close to edge");
                            }
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
