extern crate amethyst;
use amethyst::ecs::SystemData;
use amethyst::{
    assets::AssetStorage,
    core::math::Vector3,
    core::{
        geometry::Plane,
        math::{Point2, Vector2},
        transform::Transform,
        Named,
    },
    derive::SystemDesc,
    ecs::{Entities, Join, Read, ReadExpect, ReadStorage, System},
    input::{InputHandler, StringBindings},
    renderer::{
        camera::{ActiveCamera, Camera},
        sprite::SpriteSheet,
    },
    tiles::Map,
    tiles::TileMap,
    window::ScreenDimensions,
    winit,
};
use sanity_lib::tile::RoomTile;

#[derive(SystemDesc)]
pub struct TileSelectSystem;

impl<'s> System<'s> for TileSelectSystem {
    // The same BindingTypes from the InputBundle needs to be inside the InputHandler
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, Transform>,
        ReadStorage<'s, Camera>,
        ReadStorage<'s, TileMap<RoomTile>>,
        ReadStorage<'s, Named>,
        Read<'s, AssetStorage<SpriteSheet>>,
        ReadExpect<'s, ScreenDimensions>,
        Read<'s, ActiveCamera>,
        Read<'s, InputHandler<StringBindings>>,
    );
    fn run(
        &mut self,
        (
            entities,
            transforms,
            cameras,
            tilemaps,
            names,
            sprite_sheets,
            screen_dimensions,
            active_camera,
            input,
        ): Self::SystemData,
    ) {
        // Gets mouse coordinates
        if input.mouse_button_is_down(winit::MouseButton::Left) {
            if let Some((x, y)) = input.mouse_position() {
                // Get the active camera if it is spawned and ready
                let mut camera_join = (&cameras, &transforms).join();
                if let Some((camera, camera_transform)) = active_camera
                    .entity
                    .and_then(|a| camera_join.get(a, &entities))
                    .or_else(|| camera_join.next())
                {
                    println!("Casting Ray");
                    // Project a ray from the camera to the 0z axis
                    let ray = camera.screen_ray(
                        Point2::new(x, y),
                        Vector2::new(screen_dimensions.width(), screen_dimensions.height()),
                        camera_transform,
                    );
                    let distance = ray.intersect_plane(&Plane::with_z(0.0)).unwrap();
                    let mouse_world_position = ray.at_distance(distance);

                    // Find any sprites which the mouse is currently inside

                    for (tilemap, transform) in (&tilemaps, &transforms).join() {
                        let tile_pos = tilemap.to_tile(
                            &Vector3::new(mouse_world_position.x, mouse_world_position.y, 0.),
                            None,
                        );
                        println!("{:?}", tile_pos);
                    }
                }
            }
        }
    }
}
