use amethyst::{
    assets::{AssetStorage, Handle, Loader},
    ecs::{prelude::Entity, World},
    prelude::WorldExt,
    renderer::{ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
};

pub fn load_sprite_sheet(world: &World, png_path: &str, ron_path: &str) -> Handle<SpriteSheet> {
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
        (),
        &world.read_resource::<AssetStorage<SpriteSheet>>(),
    )
}

pub trait Sprited {
    fn new_sprite(&self) -> SpriteRender;
}

pub struct Bullets {
    pub sheet: Handle<SpriteSheet>,
}

impl Sprited for Bullets {
    fn new_sprite(&self) -> SpriteRender {
        SpriteRender::new(self.sheet.clone(), 0)
    }
}
