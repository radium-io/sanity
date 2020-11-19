use amethyst::{
    assets::{AssetStorage, Handle, Loader, ProgressCounter, RonFormat},
    core::{
        math::{Point2, Vector3},
        Named, Parent, Transform,
    },
    ecs::prelude::*,
    input::{is_close_requested, is_key_down},
    prelude::*,
    renderer::{camera::Camera, SpriteRender, Transparent},
    tiles::{Map, TileMap},
    ui::UiCreator,
    window::ScreenDimensions,
    winit,
};
use direction::Coord;
use sanity_lib::tile::RoomTile;
use std::fmt::Debug;
use strum::EnumCount;

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

fn init_camera(world: &mut World, player: Entity) {
    let (width, height) = {
        let dim = world.read_resource::<ScreenDimensions>();
        (dim.width(), dim.height())
    };

    world
        .create_entity()
        .with(Camera::standard_2d(width / 2., height / 2.))
        .with(Transform::from(Vector3::new(0., 0., 10.)))
        .with(Parent { entity: player })
        .build();
}

// FIXME: allow other tilesets
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

    world
        .create_entity()
        .with(map)
        .with(pairs)
        .with(Transform::default())
        .build();
}

// FIXME: allow other character sprites
fn init_player(world: &mut World) -> Entity {
    let sprite_sheet = crate::resource::load_sprite_sheet(
        &world,
        "sprites/Space Cadet.png",
        "sprites/Space Cadet.ron",
    );
    let mut t = Transform::default();
    t.move_forward(1.);
    t.move_up(8.);
    world
        .create_entity()
        .with(crate::component::Player)
        .with(SpriteRender::new(sprite_sheet.clone(), 0))
        .with(Transparent)
        .with(t)
        .build()
}

impl RoomState {
    fn gen_map_exec(&self, world: &mut World) {
        world.exec(
            |(mut maps, pairs, assets, players, transforms): (
                WriteStorage<'_, TileMap<RoomTile>>,
                ReadStorage<'_, sanity_lib::assets::PairsHandle>,
                Read<'_, AssetStorage<sanity_lib::assets::Pairs>>,
                ReadStorage<'_, crate::component::Player>,
                ReadStorage<'_, Transform>,
            )| {
                for (_, transform) in (&players, &transforms).join() {
                    for (map, pair) in (&mut maps, &pairs).join() {
                        if let Ok(pos) =
                            map.to_tile(&transform.translation().xy().to_homogeneous(), None)
                        {
                            if pos.xy() < Point2::new(self.width - 3, self.height - 3)
                                && pos.xy() > Point2::new(2, 2)
                            {
                                crate::map::gen_map(
                                    map,
                                    assets.get(pair).unwrap(),
                                    self.width,
                                    self.height,
                                    Coord::new(pos.x as i32, pos.y as i32), // FIXME: not using this any more so map gen can be borken
                                );
                            }
                        }
                    }
                }
            },
        );
    }
}

impl SimpleState for RoomState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let StateData { world, .. } = data;

        // register components, may be able to remove if used by system
        world.register::<Named>();
        world.register::<Handle<sanity_lib::assets::Pairs>>();

        // insert resources in to world
        world.insert(crate::resource::Bullets {
            sheet: crate::resource::load_sprite_sheet(
                &world,
                "sprites/bullets.png",
                "sprites/bullets.ron",
            ),
        });

        world.insert(crate::resource::Enemies {
            sheet: crate::resource::load_sprite_sheet(
                &world,
                "sprites/Slime Sprite Sheet.png",
                "sprites/slime.ron",
            ),
        });

        let player = init_player(world);
        init_camera(world, player);
        init_map(self.width, self.height, world, &mut self.progress_counter);

        // FIXME: move to global state?
        world.exec(|mut creator: UiCreator<'_>| {
            creator.create("ui/fps.ron", ());
        });
    }

    fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        if self.progress_counter.is_complete() && self.map_generation < 1 {
            self.gen_map_exec(data.world);
            self.map_generation += 1;

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
                self.gen_map_exec(data.world);
                self.map_generation += 1;

                Trans::None
            } else {
                Trans::None
            }
        } else {
            Trans::None
        }
    }
}
