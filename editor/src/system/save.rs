use amethyst::{
    assets::AssetStorage,
    core::math::Point3,
    derive::SystemDesc,
    ecs::{Join, Read, ReadStorage, System, SystemData, WriteStorage},
    input::{InputHandler, StringBindings},
    tiles::{Map, MapStorage, TileMap},
    utils::application_root_dir,
    winit,
};
use sanity_lib::{assets::Pairs, tile::RoomTile};

#[derive(SystemDesc, Default)]
pub struct SaveSystem {
    saving: bool,
    pairs: Option<Pairs>,
}

impl<'s> System<'s> for SaveSystem {
    // The same BindingTypes from the InputBundle needs to be inside the InputHandler
    type SystemData = (
        WriteStorage<'s, TileMap<RoomTile>>,
        ReadStorage<'s, sanity_lib::assets::PairsHandle>,
        Read<'s, InputHandler<StringBindings>>,
        Read<'s, AssetStorage<sanity_lib::assets::Pairs>>,
        ReadStorage<'s, crate::state::SavePath>,
    );

    fn run(
        &mut self,
        (mut tilemaps, pairs_handles, input, pairs_storage, save_paths): Self::SystemData,
    ) {
        for (tilemap, pairs, save_path) in (&mut tilemaps, &pairs_handles, &save_paths).join() {
            let dim = *tilemap.dimensions();

            if self.pairs.is_none() && pairs_storage.get(pairs).is_some() {
                self.pairs = Some(pairs_storage.get(pairs).unwrap().clone());

                // load pairs to RoomTile
                for y in 0..dim.y {
                    for x in 0..dim.x {
                        if let Some(t) = tilemap.get_mut(&Point3::new(x, y, 0)) {
                            t.candidates.s = self
                                .pairs
                                .clone()
                                .unwrap()
                                .ns
                                .into_iter()
                                .filter(|p| p.0 == t.sprite.unwrap())
                                .map(|p| p.1)
                                .collect();

                            t.candidates.e = self
                                .pairs
                                .clone()
                                .unwrap()
                                .we
                                .into_iter()
                                .filter(|p| p.0 == t.sprite.unwrap())
                                .map(|p| p.1)
                                .collect();
                        }
                    }
                }
            }

            if self.pairs.is_some() {
                let mut p = Pairs::default();

                for y in 0..dim.y {
                    for x in 0..dim.x {
                        if let Some(t) = tilemap.get(&Point3::new(x, y, 0)) {
                            for s in t.candidates.s.as_slice() {
                                p.ns.push((t.sprite.unwrap(), *s));
                            }

                            for e in t.candidates.e.as_slice() {
                                p.we.push((t.sprite.unwrap(), *e))
                            }
                        }
                    }
                }

                p.walkable = self.pairs.clone().unwrap().walkable;
                p.null = self.pairs.clone().unwrap().null;

                if input.key_is_down(winit::VirtualKeyCode::S) && !self.saving {
                    let s =
                        ron::ser::to_string_pretty(&p, ron::ser::PrettyConfig::default()).unwrap();
                    let save = application_root_dir()
                        .unwrap()
                        .parent()
                        .unwrap()
                        .join("assets")
                        .join(save_path.0.clone());

                    std::fs::write(save, s).unwrap();
                }

                self.pairs = Some(p);
            }
        }
    }
}
