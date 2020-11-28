use amethyst::{
    assets::{AssetStorage, Handle, Loader, Prefab, ProgressCounter, RonFormat},
    core::Named,
    prelude::*,
    renderer::SpriteSheet,
};

use crate::{gamedata::CustomGameData, MyPrefabData};

use super::RoomState;

#[derive(Default)]
pub struct LoadingState {
    progress_counter: ProgressCounter,
    player: Option<Handle<Prefab<MyPrefabData>>>,
    map: Option<Handle<SpriteSheet>>,
    pairs: Option<Handle<sanity_lib::assets::Pairs>>,
}

impl<'a, 'b> State<crate::gamedata::CustomGameData<'a, 'b>, StateEvent> for LoadingState {
    fn on_start(&mut self, data: StateData<'_, CustomGameData<'a, 'b>>) {
        let StateData { mut world, .. } = data;

        // register components, may be able to remove if used by system
        world.register::<Named>();
        world.register::<Handle<sanity_lib::assets::Pairs>>();

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
        let loader = world.read_resource::<Loader>();
        self.pairs = Some(loader.load(
            "Dungeon_Tileset.pairs.ron",
            RonFormat,
            &mut self.progress_counter,
            &world.read_resource::<AssetStorage<sanity_lib::assets::Pairs>>(),
        ));
    }

    fn update(
        &mut self,
        data: StateData<'_, CustomGameData<'_, '_>>,
    ) -> Trans<CustomGameData<'a, 'b>, StateEvent> {
        data.data.update(&data.world, false);

        if self.progress_counter.is_complete() {
            Trans::Switch(Box::new(RoomState {
                camera: None,
                map_generation: 0,
                width: 48,
                height: 32,
                player: self.player.take().expect("Player Loaded"),
                map_spritesheet: self.map.take().expect("Map Loaded"),
                pairs: self.pairs.take().expect("Pairs Loaded"),
            }))
        } else {
            Trans::None
        }
    }
}
