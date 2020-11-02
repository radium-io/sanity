use amethyst::{
    assets::{AssetStorage, Handle, Loader},
    core::{math::Point3, math::Vector3, Named, Parent, Transform},
    ecs::{Component, Entity, NullStorage},
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
    ui::{RenderUi, UiBundle, UiCreator, UiFinder, UiText},
    window::ScreenDimensions,
    winit,
};

#[derive(Debug)]
pub struct RoomState;

fn load_texture<N>(name: N, world: &World) -> Handle<Texture>
where
    N: Into<String>,
{
    let loader = world.read_resource::<Loader>();
    loader.load(
        name,
        ImageFormat::default(),
        (),
        &world.read_resource::<AssetStorage<Texture>>(),
    )
}

fn load_sprite_sheet(world: &World, png_path: &str, ron_path: &str) -> Handle<SpriteSheet> {
    let loader = world.read_resource::<Loader>();
    let texture_handle = load_texture(png_path, &world);
    let spritesheet_storage = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        ron_path,
        SpriteSheetFormat(texture_handle),
        (),
        &spritesheet_storage,
    )
}

fn init_camera(world: &mut World, parent: Entity, transform: Transform, camera: Camera) -> Entity {
    world
        .create_entity()
        .with(transform)
        .with(Parent { entity: parent })
        .with(camera)
        .named("camera")
        .build()
}

#[derive(Default)]
struct Player;

impl Component for Player {
    type Storage = NullStorage<Self>;
}

fn init_player(world: &mut World, sprite_sheet: &SpriteSheetHandle) -> Entity {
    let mut transform = Transform::default();
    transform.set_translation_xyz(0.0, 0.0, 0.1);
    let sprite = SpriteRender::new(sprite_sheet.clone(), 1);
    world
        .create_entity()
        .with(transform)
        .with(Player)
        .with(sprite)
        .with(Transparent)
        .named("player")
        .build()
}

use crate::tile::{RoomTile, TileType};
use rand::seq::SliceRandom;
use rand::thread_rng;

fn init_map(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>) {
    let width = 20;
    let height = 20;

    let mut map = TileMap::<RoomTile>::new(
        Vector3::new(width, height, 1), // The dimensions of the map
        Vector3::new(16, 16, 1),        // The dimensions of each tile
        Some(sprite_sheet_handle),
    );
    let transform = Transform::default();

    println!("Building map.");

    let mut rng = thread_rng();
    for y in 0..height {
        for x in 0..width {
            let mut tile = map.get_mut(&Point3::new(x, y, 0)).unwrap();
            tile.sprite = [TileType::Floor, TileType::WallN].choose(&mut rng).copied();
        }
    }

    world.create_entity().with(map).with(transform).build();
}

impl SimpleState for RoomState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        world.register::<Named>();
        world.register::<Player>();

        let circle_sprite_sheet_handle =
            load_sprite_sheet(&world, "sprites/logo.png", "sprites/logo.ron");

        let player = init_player(world, &circle_sprite_sheet_handle);

        let (width, height) = {
            let dim = world.read_resource::<ScreenDimensions>();
            (dim.width(), dim.height())
        };

        let _camera = init_camera(
            world,
            player,
            Transform::from(Vector3::new(0.0, 0.0, 1.1)),
            Camera::standard_2d(width, height),
        );

        let spritesheet_handle = load_sprite_sheet(&world, "tiles.png", "sprites.ron");
        init_map(world, spritesheet_handle);

        // FIXME: move to global state?
        world.exec(|mut creator: UiCreator<'_>| {
            creator.create("ui/fps.ron", ());
        });
    }

    fn handle_event(
        &mut self,
        data: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        let StateData { .. } = data;
        if let StateEvent::Window(event) = &event {
            if is_close_requested(&event) || is_key_down(&event, winit::VirtualKeyCode::Escape) {
                Trans::Quit
            } else {
                Trans::None
            }
        } else {
            Trans::None
        }
    }
}
