use amethyst::{
    animation::{
        get_animation_set, AnimationBundle, AnimationCommand, AnimationControlSet, AnimationSet,
        AnimationSetPrefab, EndControl,
    },
    assets::{Handle, Processor},
    assets::{HotReloadBundle, HotReloadStrategy},
    assets::{PrefabData, PrefabLoader, PrefabLoaderSystemDesc, ProgressCounter, RonFormat},
    core::{frame_limiter::FrameRateLimitStrategy, transform::TransformBundle, Hidden},
    derive::PrefabData,
    ecs::{prelude::*, ReadStorage, SystemData, World},
    input::{InputBundle, StringBindings},
    prelude::*,
    renderer::{
        bundle::{RenderOrder, RenderPlan, Target},
        palette,
        plugins::{RenderFlat2D, RenderToWindow},
        sprite::{prefab::SpriteScenePrefab, SpriteRender},
        types::DefaultBackend,
        Backend, Factory, RenderGroupDesc, RenderPlugin, RenderingBundle, SpriteSheet, Texture,
    },
    shred::DispatcherBuilder,
    tiles::{
        CoordinateEncoder, DrawTiles2DBounds, DrawTiles2DBoundsDefault, DrawTiles2DDesc,
        MortonEncoder2D, RenderTiles2D, Tile, TileMap,
    },
    ui::{RenderUi, UiBundle},
    utils::ortho_camera::CameraOrthoSystem,
    utils::{application_root_dir, fps_counter::FpsCounterBundle},
    Error, Result,
};
use serde::{Deserialize, Serialize};
use std::{fmt::Debug, marker::PhantomData};

mod component;
mod map;
mod resource;
mod state;
mod system;

/// Loading data for one entity
#[derive(Debug, Clone, Deserialize, PrefabData)]
pub struct MyPrefabData {
    /// Information for rendering a scene with sprites
    sprite_scene: SpriteScenePrefab,
    /// Аll animations that can be run on the entity
    animation_set: AnimationSetPrefab<usize, SpriteRender>,
}

fn main() -> Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;

    let your_red: f32 = 9.;
    let your_green: f32 = 9.;
    let your_blue: f32 = 9.;
    let your_alpha: f32 = 1.;

    let (r, g, b, a) = palette::Srgba::new(
        your_red / 255.,
        your_green / 255.,
        your_blue / 255.,
        your_alpha,
    )
    .into_linear()
    .into_components();

    let game_data = GameDataBuilder::default()
        .with_system_desc(
            PrefabLoaderSystemDesc::<MyPrefabData>::default(),
            "scene_loader",
            &[],
        )
        .with(CameraOrthoSystem::default(), "ortho_camera_system", &[])
        .with_bundle(HotReloadBundle::new(HotReloadStrategy::every(2)))?
        .with_bundle(AnimationBundle::<usize, SpriteRender>::new(
            "sprite_animation_control",
            "sprite_sampler_interpolation",
        ))?
        .with_bundle(TransformBundle::new())?
        .with_bundle(
            InputBundle::<StringBindings>::new()
                .with_bindings_from_file(&app_root.join("config/input.ron"))?,
        )?
        .with_bundle(UiBundle::<StringBindings>::new())?
        .with_bundle(FpsCounterBundle::default())?
        .with(system::fps::FPSSystem::default(), "fps_system", &[])
        .with(
            system::visibility::VisibilitySystem::default(),
            "visibility_system",
            &[],
        )
        .with(
            system::shooting::ShootingSystem::default(),
            "shooting_system",
            &["input_system"],
        )
        .with(
            system::player::PlayerSystem::default(),
            "player_system",
            &["input_system"],
        )
        .with(
            system::movement::MovementSystem::default(),
            "movement_system",
            &["input_system", "player_system", "shooting_system"],
        )
        .with(
            system::collision::CollisionSystem::default(),
            "collision_system",
            &["movement_system"],
        )
        .with(
            system::enemies::EnemySystem::default(),
            "enemy_system",
            &["movement_system"],
        )
        .with(Processor::<sanity_lib::assets::Pairs>::new(), "", &[])
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(app_root.join("config/display_config.ron"))?
                        .with_clear([r, g, b, a]),
                )
                .with_plugin(RenderUi::default())
                .with_plugin(RenderFlat2D::default())
                .with_plugin(RenderTiles2D::<sanity_lib::tile::FloorTile>::default())
                .with_plugin(RenderTiles2DTransparent::<sanity_lib::tile::RoomTile>::default()),
        )?;

    let mut game = Application::build(
        app_root.parent().unwrap().join("assets"),
        state::room::RoomState::new(48, 32),
    )?
    .with_frame_limit(FrameRateLimitStrategy::Yield, 101)
    .build(game_data)?;

    game.run();

    Ok(())
}

/// A `RenderPlugin` for rendering a 2D Tiles entity.
#[derive(Clone, Default)]
pub struct RenderTiles2DTransparent<
    T: Tile,
    E: CoordinateEncoder = MortonEncoder2D,
    Z: DrawTiles2DBounds = DrawTiles2DBoundsDefault,
> {
    target: Target,
    _marker: PhantomData<(T, E, Z)>,
}

impl<T: Tile, E: CoordinateEncoder, Z: DrawTiles2DBounds> Debug
    for RenderTiles2DTransparent<T, E, Z>
{
    fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl<T: Tile, E: CoordinateEncoder, Z: DrawTiles2DBounds> RenderTiles2DTransparent<T, E, Z> {
    /// Select render target on which Tiles should be rendered.
    #[must_use]
    pub fn with_target(mut self, target: Target) -> Self {
        self.target = target;
        self
    }
}

type SetupData<'a, T, E> = (
    ReadStorage<'a, Handle<SpriteSheet>>,
    ReadStorage<'a, Handle<Texture>>,
    ReadStorage<'a, Hidden>,
    ReadStorage<'a, TileMap<T, E>>,
);

impl<B: Backend, T: Tile, E: CoordinateEncoder, Z: DrawTiles2DBounds> RenderPlugin<B>
    for RenderTiles2DTransparent<T, E, Z>
{
    fn on_build<'a, 'b>(
        &mut self,
        world: &mut World,
        _builder: &mut DispatcherBuilder<'a, 'b>,
    ) -> Result<()> {
        SetupData::<T, E>::setup(world);

        Ok(())
    }

    fn on_plan(
        &mut self,
        plan: &mut RenderPlan<B>,
        _factory: &mut Factory<B>,
        _res: &World,
    ) -> Result<()> {
        plan.extend_target(self.target, |ctx| {
            ctx.add(
                RenderOrder::Transparent, // FIXME: I want some tiles behind player and some above
                DrawTiles2DDesc::<T, E, Z>::default().builder(),
            )?;
            Ok(())
        });
        Ok(())
    }
}
