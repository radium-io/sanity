use amethyst::{
    assets::{AssetStorage, Handle, Loader, Progress, ProgressCounter},
    prelude::*,
    renderer::{
        formats::texture::ImageFormat,
        sprite::{SpriteSheet, SpriteSheetFormat},
        Texture,
    },
};

mod edit;
mod load;

pub use edit::EditState;
pub use load::LoadState;

pub fn load_sprite_sheet<P>(
    world: &World,
    png_path: &str,
    ron_path: &str,
    progress: P,
) -> Handle<SpriteSheet>
where
    P: Progress,
{
    let loader = world.read_resource::<Loader>();
    let texture_handle = loader.load(
        png_path,
        ImageFormat::default(),
        (),
        &world.read_resource::<AssetStorage<Texture>>(),
    );
    loader.load(
        ron_path,
        SpriteSheetFormat(texture_handle),
        progress,
        &world.read_resource::<AssetStorage<SpriteSheet>>(),
    )
}
