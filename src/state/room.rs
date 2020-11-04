use crate::tile::RoomTile;
use amethyst::renderer::sprite::Sprites;
use amethyst::{
    assets::{AssetStorage, Format, Handle, Loader, PrefabData, ProgressCounter, RonFormat},
    core::{math::Point3, math::Vector3, Named, Parent, Transform},
    derive::PrefabData,
    ecs::{Component, Entity, Join, NullStorage},
    input::{is_close_requested, is_key_down},
    prelude::*,
    renderer::{
        camera::Camera,
        formats::texture::ImageFormat,
        sprite::{SpriteRender, SpriteSheet, SpriteSheetFormat, SpriteSheetHandle},
        transparent::Transparent,
        Texture,
    },
    tiles::{MapStorage, TileMap},
    ui::UiCreator,
    window::ScreenDimensions,
    winit, Error,
};
use rand::prelude::*;
use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;

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

fn init_camera(world: &mut World, transform: Transform, camera: Camera) -> Entity {
    world
        .create_entity()
        .with(transform)
        .with(camera)
        .named("camera")
        .build()
}

fn gen_map(map: &mut TileMap<RoomTile>, pairs: &crate::assets::Pairs, width: u32, height: u32) {
    let mut rng = thread_rng();

    // 1. seed a random tile to start the room
    let mut x = 0;
    let mut y = 0;
    let mut sprite = rng.gen_range(0, 3);

    // 2. calc possible neighborhood of that tile
    while x < width {
        let mut tile = map
            .get_mut(&Point3::new(x, y, 0))
            .expect(&format!("{:?}", x));

        // based on sprite to the left
        tile.sprite = pairs
            .clone()
            .we
            .into_iter()
            .filter(|p| p.0 == sprite)
            .collect::<Vec<(usize, usize)>>()
            .choose(&mut rng)
            .map(|p| p.1)
            .or(Some(0));

        sprite = tile.sprite.clone().unwrap();

        let mut above = sprite;

        while y < height {
            y = y + 1;

            let mut below = map.get_mut(&Point3::new(x, y, 0)).unwrap();
            // based on sprite above
            below.sprite = pairs
                .clone()
                .ns
                .into_iter()
                .filter(|p| p.0 == above)
                .collect::<Vec<(usize, usize)>>()
                .choose(&mut rng)
                .map(|p| p.1)
                .or(Some(0));

            above = below.sprite.clone().unwrap();
        }
        y = 0;
        x = x + 1;
    }
}

use amethyst::ecs::prelude::*;

impl SimpleState for RoomState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let StateData { world, .. } = data;

        world.register::<Named>();
        world.register::<Handle<crate::assets::Pairs>>();

        let (width, height) = {
            let dim = world.read_resource::<ScreenDimensions>();
            (dim.width(), dim.height())
        };

        let _camera = init_camera(
            world,
            //player,
            Transform::from(Vector3::new(0.0, 0.0, 1.1)),
            Camera::standard_2d(width, height),
        );

        let spritesheet_handle =
            load_sprite_sheet(&world, "Dungeon_Tileset.png", "Dungeon_Tileset.ron");
        let width = 20;
        let height = 20;

        let map = TileMap::<RoomTile>::new(
            Vector3::new(width, height, 1), // The dimensions of the map
            Vector3::new(32, 32, 1),        // The dimensions of each tile
            Some(spritesheet_handle),
        );
        let transform = Transform::default();

        // load the tile pairs for this tileset
        let pairs = {
            let loader = world.read_resource::<Loader>();
            loader.load(
                "Dungeon_Tileset.pairs.ron",
                RonFormat,
                &mut self.progress_counter,
                &world.read_resource::<AssetStorage<crate::assets::Pairs>>(),
            )
        };

        world
            .create_entity()
            .with(map)
            .with(pairs)
            .with(transform)
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
                    ReadStorage<'_, crate::assets::PairsHandle>,
                    Read<'_, AssetStorage<crate::assets::Pairs>>,
                )| {
                    for (map, pair) in (&mut maps, &pairs).join() {
                        gen_map(map, assets.get(pair).unwrap(), 20, 20);
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
                        ReadStorage<'_, crate::assets::PairsHandle>,
                        Read<'_, AssetStorage<crate::assets::Pairs>>,
                    )| {
                        for (map, pair) in (&mut maps, &pairs).join() {
                            gen_map(map, assets.get(pair).unwrap(), 20, 20);
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
