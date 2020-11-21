use amethyst::{
    assets::{AssetStorage, Handle, Loader, ProgressCounter, RonFormat},
    core::{
        math::{Point2, Point3, Vector3},
        Hidden, Named, Parent, Transform,
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
use bracket_pathfinding::prelude::Point;
use direction::Coord;
use sanity_lib::tile::FloorTile;
use sanity_lib::tile::RoomTile;
use std::fmt::Debug;
use strum::EnumCount;

#[derive(Default)]
pub struct RoomState {
    progress_counter: ProgressCounter,
    map_generation: usize,
    width: u32,
    height: u32,
    pairs: Option<Handle<sanity_lib::assets::Pairs>>,
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
        .with(Transform::from(Vector3::new(0., 0., 100.)))
        .with(Parent { entity: player })
        .build();
}

// FIXME: allow other character sprites
fn init_player(world: &mut World, start: Point, prog: &mut ProgressCounter) -> Entity {
    let mut t = Transform::default();
    t.move_up(8.);
    t.move_forward(60.);
    let prefab = crate::resource::load_anim_prefab(world, "sprites/Space Cadet.anim.ron", prog);
    world
        .create_entity()
        .with(Transparent)
        .with(crate::component::Player)
        .with(crate::component::Position { pos: start })
        .with(prefab)
        .with(t)
        .build()
}

impl RoomState {
    // FIXME: allow other tilesets
    fn init_map(&mut self, world: &mut World) {
        let spritesheet_handle = crate::resource::load_sprite_sheet(
            &world,
            "Dungeon_Tileset.png",
            "Dungeon_Tileset.ron",
            &mut self.progress_counter,
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

        let floor = TileMap::<FloorTile>::new(
            Vector3::new(self.width, self.height, 1), // The dimensions of the map
            Vector3::new(32, 32, 1),                  // The dimensions of each tile
            Some(spritesheet_handle.clone()),
        );

        let mut t = Transform::default();
        t.move_forward(10.);
        world
            .create_entity()
            .with(floor)
            .with(t)
            .named("floor")
            .build();

        let walls = TileMap::<RoomTile>::new(
            Vector3::new(self.width, self.height, 1), // The dimensions of the map
            Vector3::new(32, 32, 1),                  // The dimensions of each tile
            Some(spritesheet_handle.clone()),
        );

        let mut t = Transform::default();
        t.move_forward(50.);
        world
            .create_entity()
            .with(walls)
            .with(t)
            .named("walls")
            .build();

        self.pairs = Some(pairs);
    }

    fn gen_map_exec(&self, world: &mut World) {
        world.exec(
            |(mut wall_maps, mut floor_maps, assets, players, positions, names): (
                WriteStorage<'_, TileMap<RoomTile>>,
                WriteStorage<'_, TileMap<FloorTile>>,
                Read<'_, AssetStorage<sanity_lib::assets::Pairs>>,
                ReadStorage<'_, crate::component::Player>,
                ReadStorage<'_, crate::component::Position>,
                ReadStorage<'_, Named>,
            )| {
                for floor in (&mut floor_maps).join() {
                    for walls in (&mut wall_maps).join() {
                        for (_, pos) in (&players, &positions).join() {
                            if pos.xy() < Point2::new(self.width - 3, self.height - 3)
                                && pos.xy() > Point2::new(2, 2)
                            {
                                let pairs = &self.pairs.as_ref().unwrap().clone();
                                crate::map::gen_map(
                                    walls,
                                    floor,
                                    assets.get(&pairs).unwrap(),
                                    Coord::new(pos.pos.x as i32, pos.pos.y as i32),
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
        let StateData { mut world, .. } = data;

        // register components, may be able to remove if used by system
        world.register::<Named>();
        world.register::<Handle<sanity_lib::assets::Pairs>>();

        // insert resources in to world
        world.insert(crate::resource::Bullets {
            sheet: crate::resource::load_sprite_sheet(
                &world,
                "sprites/bullets.png",
                "sprites/bullets.ron",
                &mut self.progress_counter,
            ),
        });

        let anims = crate::resource::load_anim_prefab(
            &mut world,
            "sprites/slime.anim.ron",
            &mut self.progress_counter,
        );
        world.insert(crate::resource::Enemies { anims });

        let player = init_player(
            world,
            Point::new(self.width / 2, self.height / 2),
            &mut self.progress_counter,
        );
        init_camera(world, player);
        self.init_map(world);

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
