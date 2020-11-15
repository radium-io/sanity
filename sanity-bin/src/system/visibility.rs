use amethyst::{
    core::math::Point3,
    derive::SystemDesc,
    ecs::Join,
    ecs::{
        prelude::{Entity, System, SystemData, WriteStorage},
        ReadStorage,
    },
    renderer::palette,
    tiles::Map,
    tiles::MapStorage,
    tiles::TileMap,
};
use bracket_pathfinding::prelude::{field_of_view_set, Point};
use sanity_lib::{map::SanityMap, tile::RoomTile};

#[derive(Default, SystemDesc)]
pub struct VisibilitySystem {}

impl<'a> System<'a> for VisibilitySystem {
    type SystemData = (
        WriteStorage<'a, TileMap<RoomTile>>,
        ReadStorage<'a, sanity_lib::player::Player>,
    );

    fn run(&mut self, (mut tilemaps, players): Self::SystemData) {
        for player in (&players).join() {
            for tilemap in (&mut tilemaps).join() {
                let dim = tilemap.dimensions().clone();
                let clone = tilemap.clone();
                let my_map = SanityMap(&clone);
                let fov = field_of_view_set(player.pos(), 4, &my_map);

                for x in 0..dim.x {
                    for y in 0..dim.y {
                        let p = Point::new(x, y);
                        if let Some(tile) = tilemap.get_mut(&Point3::new(x, y, 0)) {
                            if !fov.contains(&p) {
                                tile.tint = palette::Srgba::new(0.1, 0.1, 0.1, 0.9);
                            } else {
                                tile.tint = palette::Srgba::new(1., 1., 1., 1.);
                            }
                        }
                    }
                }
            }
        }
    }
}
