use crate::{gamedata::CustomGameData, resource::Sprited, AnimatedSpritePrefab};
use amethyst::{
    assets::{AssetStorage, Handle, Loader, Prefab, ProgressCounter, RonFormat},
    core::{
        math::{Point2, Vector3},
        Named, Parent, Transform,
    },
    ecs::prelude::*,
    input::{is_close_requested, is_key_down},
    prelude::*,
    renderer::{camera::Camera, SpriteSheet, Transparent},
    tiles::{Map, TileMap},
    ui::UiCreator,
    window::ScreenDimensions,
    winit,
};
use bracket_pathfinding::prelude::Point;
use sanity_lib::tile::{FloorTile, RoomTile};

pub struct RoomState {
    pub map_generation: usize,
    pub width: u32,
    pub height: u32,
    pub pairs: Handle<sanity_lib::assets::Pairs>,
    pub camera: Option<Entity>,
    pub walls: Option<Entity>,
    pub player: Handle<Prefab<AnimatedSpritePrefab>>,
    pub map_spritesheet: Handle<SpriteSheet>,
}

impl RoomState {
    fn init_map(&mut self, world: &mut World) {
        let map_size = Vector3::new(self.width, self.height, 1);
        let tile_size = Vector3::new(32, 32, 1);

        world
            .create_entity()
            .with(TileMap::<FloorTile>::new(
                map_size,
                tile_size,
                Some(self.map_spritesheet.clone()),
            ))
            .build();

        self.walls = Some(
            world
                .create_entity()
                .with(TileMap::<RoomTile>::new(
                    map_size,
                    tile_size,
                    Some(self.map_spritesheet.clone()),
                ))
                .build(),
        );
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

    fn init_player(&self, world: &mut World, pos: Point) -> Entity {
        let weapon = world
            .create_entity()
            .with(crate::component::Weapon {
                damage_range: (3, 8),
                ranged: true,
            })
            .named("Blaster")
            .build();

        let mut t = Transform::default();
        t.move_up(8.);

        world
            .create_entity()
            .with(Transparent)
            .with(crate::component::Player {
                weapon: Some(weapon),
                inventory: vec![],
            })
            .with(crate::component::Health {
                max: 30,
                current: 30,
            })
            .with(crate::component::Position { pos })
            .with(self.player.clone())
            .with(t)
            .build()
    }

    fn gen_map_exec(&self, world: &mut World) {
        // delete all the enemies so they respawn
        world.exec(
            |(entities, enemies): (Entities<'_>, ReadStorage<'_, crate::component::Enemy>)| {
                for (entity, _enemy) in (&entities, &enemies).join() {
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
                                crate::map::gen_map(
                                    walls,
                                    floor,
                                    assets.get(&self.pairs.clone()).unwrap(),
                                    pos.coord(),
                                );
                            }
                        }
                    }
                }
            },
        );
    }
}

impl<'a, 'b> State<crate::gamedata::CustomGameData<'a, 'b>, StateEvent> for RoomState {
    fn on_start(&mut self, data: StateData<'_, CustomGameData<'a, 'b>>) {
        let StateData { mut world, .. } = data;

        let player = self.init_player(world, Point::new(self.width / 2, self.height / 2));
        self.init_camera(world, player);
        self.init_map(world);

        self.gen_map_exec(world);
        self.map_generation += 1;

        world.exec(|mut creator: UiCreator<'_>| {
            creator.create("ui/hud.ron", ());
        });
    }

    fn on_resume(&mut self, data: StateData<'_, CustomGameData<'a, 'b>>) {
        let StateData { world, .. } = data;

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
                |(entities, positions): (
                    Entities<'_>,
                    ReadStorage<'_, crate::component::Position>,
                )| {
                    for (entity, _) in (&entities, &positions).join() {
                        entities.delete(entity); // also deletes camera child
                    }
                },
            );
            world.maintain();
            let player = self.init_player(world, Point::new(self.width / 2, self.height / 2));
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

        let sanity_res = data.world.read_resource::<crate::state::Sanity>();
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
            } else {
                Trans::None
            }
        } else {
            Trans::None
        }
    }
}
