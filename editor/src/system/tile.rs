extern crate amethyst;
use amethyst::{
    core::math::{Point3, Vector3},
    core::{
        geometry::Plane,
        math::{Point2, Vector2},
        transform::Transform,
    },
    derive::SystemDesc,
    ecs::{Entities, Join, Read, ReadExpect, ReadStorage, System, WriteStorage},
    input::{InputHandler, StringBindings},
    renderer::camera::{ActiveCamera, Camera},
    renderer::palette,
    renderer::palette::Srgba,
    tiles::Map,
    tiles::TileMap,
    window::ScreenDimensions,
    winit,
};
use amethyst::{ecs::SystemData, tiles::MapStorage};
use sanity_lib::tile::RoomTile;

use crate::state::edit::Selected;

#[derive(SystemDesc, Default)]
pub struct TileSelectSystem {
    left_down: bool,
    right_down: bool,
}

fn tint_tile(tilemap: &mut TileMap<RoomTile>, tile_pos: &Point3<u32>, tint: palette::Srgba) {
    if let Some(tile) = tilemap.get_mut(tile_pos) {
        tile.tint = tint;
    }
}

impl<'s> System<'s> for TileSelectSystem {
    // The same BindingTypes from the InputBundle needs to be inside the InputHandler
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, Transform>,
        ReadStorage<'s, Camera>,
        WriteStorage<'s, TileMap<RoomTile>>,
        ReadExpect<'s, ScreenDimensions>,
        Read<'s, ActiveCamera>,
        Read<'s, InputHandler<StringBindings>>,
        WriteStorage<'s, crate::state::edit::Selected>,
    );

    fn run(
        &mut self,
        (
            entities,
            transforms,
            cameras,
            mut tilemaps,
            screen_dimensions,
            active_camera,
            input,
            mut selected,
        ): Self::SystemData,
    ) {
        // Gets mouse coordinates
        if let Some((x, y)) = input.mouse_position() {
            // Get the active camera if it is spawned and ready
            let mut camera_join = (&cameras, &transforms).join();
            if let Some((camera, camera_transform)) = active_camera
                .entity
                .and_then(|a| camera_join.get(a, &entities))
                .or_else(|| camera_join.next())
            {
                // Project a ray from the camera to the 0z axis
                let ray = camera.screen_ray(
                    Point2::new(x, y),
                    Vector2::new(screen_dimensions.width(), screen_dimensions.height()),
                    camera_transform,
                );

                if let Some(distance) = ray.intersect_plane(&Plane::with_z(0.0)) {
                    let mouse_world_position = ray.at_distance(distance);

                    // Find any sprites which the mouse is currently inside
                    for (entity, tilemap) in (&entities, &mut tilemaps).join() {
                        match tilemap.to_tile(
                            &Vector3::new(mouse_world_position.x, mouse_world_position.y, 0.),
                            None,
                        ) {
                            Ok(tile_pos) => {
                                if input.mouse_button_is_down(winit::MouseButton::Left) {
                                    if !self.left_down {
                                        selected.insert(
                                            entity,
                                            crate::state::edit::Selected(Some(tile_pos)),
                                        );
                                    }
                                    self.left_down = true;
                                } else if let Some(Selected(Some(selected_pos))) =
                                    selected.get(entity)
                                {
                                    self.left_down = false;

                                    if input.action_is_down("east").unwrap_or(false) {
                                        if !self.right_down {
                                            let index = tile_pos.x as usize
                                                + (tile_pos.y * tilemap.dimensions().x) as usize;
                                            let prev = tilemap.get_mut(&selected_pos).unwrap();

                                            if prev.candidates.e.contains(&index) {
                                                prev.candidates.e.retain(|x| *x != index);
                                            } else {
                                                prev.candidates.e.push(index);
                                            }
                                        }
                                        self.right_down = true;
                                    } else if input.mouse_button_is_down(winit::MouseButton::Right)
                                    {
                                        if !self.right_down {
                                            let index = tile_pos.x as usize
                                                + (tile_pos.y * tilemap.dimensions().x) as usize;
                                            let prev = tilemap.get_mut(&selected_pos).unwrap();

                                            if prev.candidates.s.contains(&index) {
                                                prev.candidates.s.retain(|x| *x != index);
                                            } else {
                                                prev.candidates.s.push(index);
                                            }
                                        }

                                        self.right_down = true;
                                    } else {
                                        self.right_down = false;
                                    }
                                }
                            }
                            Err(_) => {}
                        }

                        // tint all tiles that are candidates (so selecting new tile will show candidates)
                        if let Some(Selected(Some(selected_pos))) = selected.get(entity) {
                            let s = tilemap.get(&selected_pos).unwrap().clone();
                            let size = tilemap.dimensions();
                            let sprite = s.sprite.unwrap() as u32;

                            for idx in 0..(size.x * size.y) {
                                let pos = Point3::new(
                                    idx % tilemap.dimensions().x,
                                    idx / tilemap.dimensions().x,
                                    0,
                                );

                                let i = &(idx as usize);

                                if s.candidates.s.contains(i) && s.candidates.e.contains(i) {
                                    tint_tile(tilemap, &pos, Srgba::new(0.0, 1.0, 1.0, 0.7));
                                } else if s.candidates.s.contains(i) {
                                    tint_tile(tilemap, &pos, Srgba::new(0.0, 1.0, 0.0, 0.7));
                                } else if s.candidates.e.contains(i) {
                                    tint_tile(tilemap, &pos, Srgba::new(0.0, 0.0, 1.0, 0.7));
                                } else if idx == sprite {
                                    tint_tile(tilemap, &pos, Srgba::new(1.0, 0.0, 0.0, 0.7));
                                } else {
                                    tint_tile(tilemap, &pos, Srgba::new(1.0, 1.0, 1.0, 1.));
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
