use amethyst::{
    assets::{AssetStorage, Handle, Loader, ProgressCounter, RonFormat},
    core::{
        math::{Point3, Vector3},
        Transform,
    },
    ecs::{Component, Entity, HashMapStorage},
    input::{is_close_requested, is_key_down},
    prelude::*,
    renderer::camera::Camera,
    tiles::{MapStorage, TileMap},
    ui::UiCreator,
    utils::ortho_camera::{CameraNormalizeMode, CameraOrtho, CameraOrthoWorldCoordinates},
    window::ScreenDimensions,
    winit,
};
use sanity_lib::tile::RoomTile;

#[derive(Debug, Default)]
pub struct EditState {
    pub progress_counter: ProgressCounter,
    pub png: String,
    pub ron: String,
    pub pairs: String,
    pub map: Option<Entity>,
}

pub struct SavePath(pub String);
impl Component for SavePath {
    type Storage = HashMapStorage<Self>;
}

#[derive(Debug)]
pub struct Selected(pub Option<Point3<u32>>);
impl Component for Selected {
    type Storage = HashMapStorage<Self>;
}

impl SimpleState for EditState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let StateData { world, .. } = data;

        // register any components that aren't in a system
        world.register::<Handle<sanity_lib::assets::Pairs>>();
        world.register::<CameraOrtho>();

        let (width, height) = {
            let dim = world.read_resource::<ScreenDimensions>();
            (dim.width(), dim.height())
        };

        // set up an ortho camera that resizes automatically
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
            .with(Transform::from(Vector3::new(0., 0., 2.)))
            .with(Camera::standard_2d(width, height))
            .with(ortho)
            .build();

        let spritesheet_handle = super::load_sprite_sheet(
            &world,
            self.png.as_ref(),
            self.ron.as_ref(),
            &mut self.progress_counter,
        );

        let mut map = TileMap::<RoomTile>::new(
            Vector3::new(16, 20, 1), // The dimensions of the map
            Vector3::new(32, 32, 1), // The dimensions of each tile
            Some(spritesheet_handle),
        );

        let mut x = 0;
        let mut y = 0;
        while y < 20 {
            while x < 16 {
                if let Some(tile) = map.get_mut(&Point3::new(x, y, 0)) {
                    tile.sprite = Some(x as usize + (y * 16) as usize);
                    tile.visible = true;
                    tile.visited = true;
                }
                x += 1;
            }
            y += 1;
            x = 0;
        }

        let pairs = {
            let loader = world.read_resource::<Loader>();
            loader.load(
                self.pairs.clone(),
                RonFormat,
                &mut self.progress_counter,
                &world.read_resource::<AssetStorage<sanity_lib::assets::Pairs>>(),
            )
        };

        let save_path = SavePath(self.pairs.clone());

        world
            .create_entity()
            .with(map)
            .with(pairs)
            .with(save_path)
            .with(Transform::default())
            .build();

        world.exec(|mut creator: UiCreator<'_>| {
            creator.create("ui/list.ron", ());
        });
    }

    fn update(&mut self, _data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        Trans::None
    }

    fn handle_event(
        &mut self,
        _data: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
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
