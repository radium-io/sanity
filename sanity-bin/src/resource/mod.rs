use amethyst::{
    assets::{AssetStorage, Handle, Loader, Prefab, PrefabLoader, ProgressCounter, RonFormat},
    ecs::World,
    prelude::WorldExt,
    renderer::{ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
};

pub fn load_sprite_sheet(
    world: &World,
    png_path: &str,
    ron_path: &str,
    prog: &mut ProgressCounter,
) -> Handle<SpriteSheet> {
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
        prog,
        &world.read_resource::<AssetStorage<SpriteSheet>>(),
    )
}

pub fn load_anim_prefab(
    world: &mut World,
    prefab_path: &str,
    prog: &mut ProgressCounter,
) -> Handle<Prefab<crate::AnimatedSpritePrefab>> {
    world.exec(|loader: PrefabLoader<'_, crate::AnimatedSpritePrefab>| {
        loader.load(prefab_path, RonFormat, prog)
    })
}

pub trait Sprited {
    fn new_sprite(&self) -> SpriteRender;
}

pub trait Animated {
    fn new_animated_sprite(&self) -> Handle<Prefab<crate::AnimatedSpritePrefab>>;
}

pub struct Bullets {
    pub sheet: Handle<SpriteSheet>,
}

impl Sprited for Bullets {
    fn new_sprite(&self) -> SpriteRender {
        SpriteRender::new(self.sheet.clone(), 0)
    }
}

pub struct Enemies {
    pub anims: Handle<Prefab<crate::AnimatedSpritePrefab>>,
}

impl Animated for Enemies {
    fn new_animated_sprite(&self) -> Handle<Prefab<crate::AnimatedSpritePrefab>> {
        self.anims.clone()
    }
}
