use amethyst::{
    assets::{AssetStorage, Handle, Loader, ProgressCounter, RonFormat},
    core::{
        math::{Point2, Point3, Vector3},
        Named, Parent, Transform,
    },
    ecs::prelude::*,
    input::{is_close_requested, is_key_down},
    prelude::*,
    renderer::{
        camera::Camera,
        debug_drawing::{DebugLines, DebugLinesComponent, DebugLinesParams},
        palette::Srgba,
        Transparent,
    },
    tiles::TileMap,
    ui::UiCreator,
    window::ScreenDimensions,
    winit,
};
use bracket_pathfinding::prelude::Point;
use direction::Coord;
use sanity_lib::tile::FloorTile;
use sanity_lib::tile::RoomTile;

#[derive(Default)]
pub struct RoomState {
    progress_counter: ProgressCounter,
    map_generation: usize,
    width: u32,
    height: u32,
    pairs: Option<Handle<sanity_lib::assets::Pairs>>,
    camera: Option<Entity>,
    map: Option<Entity>,
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

// FIXME: allow other character sprites
fn init_player(world: &mut World, start: Point, prog: &mut ProgressCounter) -> Entity {
    let mut t = Transform::default();
    t.move_up(8.);
    let prefab = crate::resource::load_anim_prefab(world, "sprites/Space Cadet.anim.ron", prog);
    let weapon = world
        .create_entity()
        .with(crate::component::Weapon {
            damage_range: (3, 8),
            ranged: true,
        })
        .named("Blaster")
        .build();
    world
        .create_entity()
        .with(Transparent)
        .with(crate::component::Player {
            weapon: Some(weapon),
        })
        .with(crate::component::Health {
            max: 30,
            current: 30,
        })
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
        world
            .create_entity()
            .with(floor)
            .with(t)
            .named("floor")
            .build();

        let walls = TileMap::<RoomTile>::new(
            Vector3::new(self.width, self.height, 1), // The dimensions of the map
            Vector3::new(32, 32, 1),                  // The dimensions of each tile
            Some(spritesheet_handle),
        );

        let mut t = Transform::default();
        world
            .create_entity()
            .with(walls)
            .with(t)
            .named("walls")
            .build();

        self.pairs = Some(pairs);
    }

    fn init_camera(&mut self, world: &mut World, player: Entity) {
        let (width, height) = {
            let dim = world.read_resource::<ScreenDimensions>();
            (dim.width(), dim.height())
        };
        self.camera = Some(
            world
                .create_entity()
                .with(Camera::standard_2d(width / 2., height / 2.))
                .with(Transform::from(Vector3::new(0., 0., 100.)))
                .with(Parent { entity: player })
                .build(),
        );
    }

    fn gen_map_exec(&self, world: &mut World) {
        world.exec(
            |(entities, enemies): (Entities<'_>, ReadStorage<'_, crate::component::Enemy>)| {
                // delete all the enemies so they respawn
                for (entity, enemy) in (&entities, &enemies).join() {
                    entities.delete(entity);
                }
            },
        );

        world.exec(
            |(mut wall_maps, mut floor_maps, assets, players, positions): (
                WriteStorage<'_, TileMap<RoomTile>>,
                WriteStorage<'_, TileMap<FloorTile>>,
                Read<'_, AssetStorage<sanity_lib::assets::Pairs>>,
                ReadStorage<'_, crate::component::Player>,
                ReadStorage<'_, crate::component::Position>,
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
use crate::gamedata::CustomGameData;

impl<'a, 'b> State<crate::gamedata::CustomGameData<'a, 'b>, StateEvent> for RoomState {
    fn on_start(&mut self, data: StateData<'_, CustomGameData<'a, 'b>>) {
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

        world.insert(crate::state::Sanity::default());

        let player = init_player(
            world,
            Point::new(self.width / 2, self.height / 2),
            &mut self.progress_counter,
        );
        self.init_camera(world, player);
        self.init_map(world);

        // FIXME: move to global state?
        world.exec(|mut creator: UiCreator<'_>| {
            creator.create("ui/fps.ron", ());
            creator.create("ui/hud.ron", ());
        });
    }

    fn on_resume(&mut self, data: StateData<'_, CustomGameData<'a, 'b>>) {
        let StateData { mut world, .. } = data;

        let mut restart = false;
        {
            let mut sanity_res = world.write_resource::<crate::state::Sanity>();
            if sanity_res.game_over {
                sanity_res.game_over = false;
                restart = true;
            }
        }

        if restart {
            world.exec(
                |(entities, players): (Entities<'_>, ReadStorage<'_, crate::component::Player>)| {
                    for (entity, player) in (&entities, &players).join() {
                        entities.delete(entity); // also deletes camera child
                    }
                },
            );
            let player = init_player(
                world,
                Point::new(self.width / 2, self.height / 2),
                &mut self.progress_counter,
            );
            self.init_camera(world, player);
            self.gen_map_exec(world);
            self.map_generation += 1;
        }
    }

    fn update(
        &mut self,
        data: StateData<'_, CustomGameData<'_, '_>>,
    ) -> Trans<CustomGameData<'a, 'b>, StateEvent> {
        data.data.update(&data.world, true);

        if self.progress_counter.is_complete() && self.map_generation < 1 {
            self.gen_map_exec(data.world);
            self.map_generation += 1;
        }

        let mut sanity_res = data.world.read_resource::<crate::state::Sanity>();
        if sanity_res.game_over {
            println!("Game Over");
            return Trans::Push(Box::new(super::gameover::GameOverState));
        }
        Trans::None
    }

    fn handle_event(
        &mut self,
        data: StateData<'_, CustomGameData<'a, 'b>>,
        event: StateEvent,
    ) -> Trans<CustomGameData<'a, 'b>, StateEvent> {
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
