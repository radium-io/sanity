use amethyst::{
    assets::{AssetStorage, Handle, Loader, Prefab, ProgressCounter, RonFormat},
    core::Named,
    ecs::{Entities, Entity},
    input::{is_close_requested, is_key_down},
    prelude::*,
    renderer::SpriteSheet,
    ui::UiCreator,
    winit,
};

use crate::{gamedata::CustomGameData, AnimatedSpritePrefab};

use super::RoomState;

#[derive(Default)]
pub struct LoadingState {
    progress_counter: ProgressCounter,
    player: Option<Handle<Prefab<AnimatedSpritePrefab>>>,
    map: Option<Handle<SpriteSheet>>,
    pairs: Option<Handle<sanity_lib::assets::Pairs>>,
    loading: Option<Entity>,
}

impl<'a, 'b> State<crate::gamedata::CustomGameData<'a, 'b>, StateEvent> for LoadingState {
    fn on_start(&mut self, data: StateData<'_, CustomGameData<'a, 'b>>) {
        let StateData { mut world, .. } = data;

        // register components, may be able to remove if used by system
        world.register::<Named>();
        world.register::<Handle<sanity_lib::assets::Pairs>>();
        world.register::<crate::component::Item>();

        world.insert(crate::state::Sanity::default());

        // insert resources in to world
        world.insert(crate::resource::Bullets {
            sheet: crate::resource::load_sprite_sheet(
                &world,
                "sprites/bullets.png",
                "sprites/bullets.ron",
                &mut self.progress_counter,
            ),
        });

        world.insert(crate::resource::Items {
            sheet: crate::resource::load_sprite_sheet(
                &world,
                "sprites/items.png",
                "sprites/items.ron",
                &mut self.progress_counter,
            ),
        });

        world.insert(crate::resource::Exits {
            sheet: crate::resource::load_sprite_sheet(
                &world,
                "Dungeon_Tileset.png",
                "Dungeon_Tileset.ron",
                &mut self.progress_counter,
            ),
        });

        let anims = crate::resource::load_anim_prefab(
            &mut world,
            "sprites/slime.anim.ron",
            &mut self.progress_counter,
        );
        world.insert(crate::resource::Enemies { anims });

        self.map = Some(crate::resource::load_sprite_sheet(
            &world,
            "Dungeon_Tileset.png",
            "Dungeon_Tileset.ron",
            &mut self.progress_counter,
        ));

        self.player = Some(crate::resource::load_anim_prefab(
            world,
            "sprites/Space Cadet.anim.ron",
            &mut self.progress_counter,
        ));

        // load the tile pairs for this tileset
        self.pairs = {
            let loader = world.read_resource::<Loader>();
            Some(loader.load(
                "Dungeon_Tileset.pairs.ron",
                RonFormat,
                &mut self.progress_counter,
                &world.read_resource::<AssetStorage<sanity_lib::assets::Pairs>>(),
            ))
        };

        world.exec(|mut creator: UiCreator<'_>| {
            creator.create("ui/fps.ron", ());
            self.loading = Some(creator.create("ui/loading.ron", ()));
        });
    }

    fn update(
        &mut self,
        data: StateData<'_, CustomGameData<'_, '_>>,
    ) -> Trans<CustomGameData<'a, 'b>, StateEvent> {
        let StateData { mut world, .. } = data;

        data.data.update(world, false);

        if self.progress_counter.is_complete() {
            world.exec(|entities: Entities<'_>| {
                entities.delete(self.loading.unwrap());
            });

            Trans::Push(Box::new(RoomState {
                camera: None,
                level: 1,
                width: crate::state::room::LEVEL_SIZES[0].0,
                height: crate::state::room::LEVEL_SIZES[0].1,
                player_anim: self.player.take().expect("Player Loaded"),
                map_spritesheet: self.map.take().expect("Map Loaded"),
                pairs: self.pairs.take().expect("Pairs Loaded"),
                player: None,
                walls: None,
                floors: None,
                hud: None,
            }))
        } else {
            Trans::None
        }
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
