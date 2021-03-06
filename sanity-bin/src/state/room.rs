use crate::{gamedata::CustomGameData, resource::Sprited, AnimatedSpritePrefab};
use amethyst::{
    assets::{AssetStorage, Handle, Loader, Prefab, ProgressCounter, RonFormat},
    core::{
        math::{Point2, Point3, Vector3},
        Hidden, Named, Parent, Transform,
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

pub static LEVEL_SIZES: &'static [(u32, u32)] =
    &[(12, 12), (24, 24), (48, 32), (32, 48), (64, 64), (8, 8)];

pub struct RoomState {
    pub level: usize,
    pub width: u32,
    pub height: u32,
    pub pairs: Handle<sanity_lib::assets::Pairs>,
    pub camera: Option<Entity>,
    pub walls: Option<Entity>,
    pub floors: Option<Entity>,
    pub player: Option<Entity>,
    pub hud: Option<Entity>,
    pub player_anim: Handle<Prefab<AnimatedSpritePrefab>>,
    pub map_spritesheet: Handle<SpriteSheet>,
}

impl RoomState {
    fn init_map(&mut self, world: &mut World) {
        let map_size = Vector3::new(self.width, self.height, 1);
        let tile_size = Vector3::new(32, 32, 1);

        self.floors = Some(
            world
                .create_entity()
                .with(TileMap::<FloorTile>::new(
                    map_size,
                    tile_size,
                    Some(self.map_spritesheet.clone()),
                ))
                .build(),
        );

        let mut transform = Transform::default();
        transform.move_backward(20.);

        self.walls = Some(
            world
                .create_entity()
                .with(TileMap::<RoomTile>::new(
                    map_size,
                    tile_size,
                    Some(self.map_spritesheet.clone()),
                ))
                .with(transform)
                .build(),
        );
    }

    fn init_camera(&mut self, world: &mut World) {
        let (width, height) = {
            let dim = world.read_resource::<ScreenDimensions>();
            (dim.width(), dim.height())
        };
        self.camera = Some(
            world
                .create_entity()
                .with(Camera::standard_2d(width / 2., height / 2.))
                .with(Transform::from(Vector3::new(0., 0., 100.)))
                .with(Parent {
                    entity: self.player.unwrap(),
                })
                .build(),
        );
    }

    fn init_player(&mut self, world: &mut World, pos: Point) {
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
        t.move_backward(1.);

        self.player = Some(
            world
                .create_entity()
                .with(crate::component::Player {
                    weapon: Some(weapon),
                    inventory: vec![],
                })
                .with(crate::component::Health {
                    max: 30,
                    current: 30,
                })
                .with(crate::component::Position {
                    pos,
                    map: self.walls.unwrap(),
                })
                .with(self.player_anim.clone())
                .with(t)
                .build(),
        );
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
                let floor = floor_maps.get_mut(self.floors.unwrap()).unwrap();
                let walls = wall_maps.get_mut(self.walls.unwrap()).unwrap();
                for (_, pos) in (&players, &positions).join() {
                    crate::map::gen_map(
                        walls,
                        floor,
                        assets.get(&self.pairs.clone()).unwrap(),
                        pos.coord(),
                    );
                }
            },
        );
    }
}

impl<'a, 'b> State<crate::gamedata::CustomGameData<'a, 'b>, StateEvent> for RoomState {
    fn on_start(&mut self, data: StateData<'_, CustomGameData<'a, 'b>>) {
        let StateData { mut world, .. } = data;

        self.init_map(world);

        let start = Point::new(self.width / 2, self.height / 2);

        if self.player.is_none() {
            self.init_player(world, start);
        } else {
            println!("Moving Player to start position");
            world.exec(
                |(mut positions, maps, mut transforms): (
                    WriteStorage<'_, crate::component::Position>,
                    WriteStorage<'_, TileMap<RoomTile>>,
                    WriteStorage<'_, Transform>,
                )| {
                    let mut pos = positions.get_mut(self.player.unwrap()).unwrap();
                    pos.map = self.walls.unwrap();
                    pos.pos = start;

                    let p = maps.get(self.walls.unwrap()).unwrap();
                    let mut t = Transform::from(
                        p.to_world(&Point3::new(pos.pos.x as u32, pos.pos.y as u32, 0), None),
                    );
                    t.move_up(8.);
                    t.move_backward(1.);
                    transforms.insert(self.player.unwrap(), t);
                },
            );
        }
        self.gen_map_exec(world);

        if self.camera.is_none() {
            self.init_camera(world);
        }

        if self.hud.is_none() {
            world.exec(|mut creator: UiCreator<'_>| {
                self.hud = Some(creator.create("ui/hud.ron", ()));
            });
        }

        let mut sanity_res = world.write_resource::<crate::state::Sanity>();
        sanity_res.level.pop();
        sanity_res.floor.pop();
        sanity_res.level.push(self.walls);
        sanity_res.floor.push(self.floors);
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
            self.init_player(world, Point::new(self.width / 2, self.height / 2));
            self.init_camera(world);
            self.gen_map_exec(world);
        }
    }

    fn update(
        &mut self,
        data: StateData<'_, CustomGameData<'_, '_>>,
    ) -> Trans<CustomGameData<'a, 'b>, StateEvent> {
        let StateData { mut world, .. } = data;

        let mut descend = false;

        {
            let sanity_res = world.read_resource::<crate::state::Sanity>();
            if sanity_res.game_over {
                println!("Game Over");
                return Trans::Push(Box::new(super::gameover::GameOverState::default()));
            }

            if sanity_res.level.len() > self.level {
                descend = true;
            }
        }

        if descend {
            println!("Descending");
            world.exec(
                |(entities, positions, mut hiddens): (
                    Entities<'_>,
                    ReadStorage<'_, crate::component::Position>,
                    WriteStorage<'_, Hidden>,
                )| {
                    hiddens.insert(self.floors.unwrap(), Hidden);
                    hiddens.insert(self.walls.unwrap(), Hidden);

                    for (entity, _) in (&entities, &positions).join() {
                        if entity != self.player.unwrap() {
                            entities.delete(entity);
                        }
                    }
                },
            );
            return Trans::Push(Box::new(RoomState {
                level: self.level + 1,
                width: LEVEL_SIZES[self.level].0,
                height: LEVEL_SIZES[self.level].1,
                player: self.player,
                camera: self.camera,
                player_anim: self.player_anim.clone(),
                map_spritesheet: self.map_spritesheet.clone(),
                pairs: self.pairs.clone(),
                walls: None,
                floors: None,
                hud: self.hud,
            }));
        }

        data.data.update(&world, true);

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
