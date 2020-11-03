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
    ui::UiCreator,
    window::ScreenDimensions,
    winit,
};

#[derive(Debug, Default)]
pub struct RoomState {
    map: Option<Entity>,
}

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

fn init_camera(
    world: &mut World,
    /*parent: Entity,*/ transform: Transform,
    camera: Camera,
) -> Entity {
    world
        .create_entity()
        .with(transform)
        //.with(Parent { entity: parent })
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
        .with(Transparent)
        .named("player")
        .build()
}

use crate::tile::RoomTile;
use rand::prelude::*;
use strum::IntoEnumIterator;

fn init_map(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>) -> Entity {
    let width = 20;
    let height = 20;

    let mut map = TileMap::<RoomTile>::new(
        Vector3::new(width, height, 1), // The dimensions of the map
        Vector3::new(32, 32, 1),        // The dimensions of each tile
        Some(sprite_sheet_handle),
    );
    let transform = Transform::default();

    gen_map(&mut map, width, height);

    world
        .create_entity()
        .with(map)
        .with(transform)
        .named("map")
        .build()
}

fn gen_map(map: &mut TileMap<RoomTile>, width: u32, height: u32) {
    let mut rng = thread_rng();

    // 1. seed a random tile to start the room
    let mut x = 0;
    let mut y = 0;
    let mut sprite = rng.gen_range(0, 3);

    // 2. calc possible neighborhood of that tile
    while y < height + 1 {
        while x < width {
            let mut tile = map
                .get_mut(&Point3::new(x, y, 0))
                .expect(&format!("{:?}", x));

            // based on sprite to the left
            tile.sprite = Some(match sprite {
                0 => 1,
                1 => [1, 2].choose(&mut rng).map(|u| *u as usize).unwrap(),
                2 => 0,
                _ => 0,
            });

            sprite = tile.sprite.clone().unwrap();

            let mut below = map.get_mut(&Point3::new(x, y + 1, 0)).unwrap();
            // based on sprite above
            below.sprite = Some(match sprite {
                0 => 16,
                1 => 17,
                2 => 18,
                16 => 32,
                17 => [33, 4, 5, 6, 8, 9, 10, 11, 12]
                    .choose(&mut rng)
                    .map(|u| *u as usize)
                    .unwrap(),
                18 => 34,
                32 => 48,
                _ => 0,
            });

            x = x + 1;
        }
        x = 0;
        y = y + 2;
    }
}

use amethyst::ecs::prelude::*;

impl SimpleState for RoomState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        world.register::<Named>();
        world.register::<Player>();

        //let circle_sprite_sheet_handle =
        //load_sprite_sheet(&world, "sprites/logo.png", "sprites/logo.ron");

        //let player = init_player(world, &circle_sprite_sheet_handle);

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
        self.map = Some(init_map(world, spritesheet_handle));

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
        if let StateEvent::Window(event) = &event {
            if is_close_requested(&event) || is_key_down(&event, winit::VirtualKeyCode::Escape) {
                Trans::Quit
            } else if is_key_down(&event, winit::VirtualKeyCode::F) {
                data.world
                    .exec(|(mut maps,): (WriteStorage<TileMap<RoomTile>>,)| {
                        gen_map(maps.get_mut(self.map.unwrap()).unwrap(), 20, 20);
                    });
                Trans::None
            } else {
                Trans::None
            }
        } else {
            Trans::None
        }
    }
}
