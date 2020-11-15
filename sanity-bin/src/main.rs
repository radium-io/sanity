use std::{fmt::Debug, marker::PhantomData};

use amethyst::{
    assets::Handle,
    assets::Processor,
    core::frame_limiter::FrameRateLimitStrategy,
    core::{transform::TransformBundle, Hidden},
    ecs::ReadStorage,
    input::{InputBundle, StringBindings},
    prelude::*,
    renderer::bundle::RenderOrder,
    renderer::bundle::RenderPlan,
    renderer::bundle::Target,
    renderer::Factory,
    renderer::RenderPlugin,
    renderer::SpriteSheet,
    renderer::Texture,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        Backend, RenderingBundle,
    },
    shred::DispatcherBuilder,
    tiles::CoordinateEncoder,
    tiles::DrawTiles2DDesc,
    tiles::MortonEncoder2D,
    tiles::{DrawTiles2DBounds, DrawTiles2DBoundsDefault, Tile, TileMap},
    ui::{RenderUi, UiBundle},
    utils::application_root_dir,
    utils::fps_counter::FpsCounterBundle,
};

use amethyst_error;

mod state;
mod system;

use amethyst::assets::{HotReloadBundle, HotReloadStrategy};
use amethyst_utils::ortho_camera::CameraOrthoSystem;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;

    let resources = app_root.parent().unwrap().join("assets");
    let display_config = app_root.join("config/display_config.ron");
    let key_bindings_path = app_root.join("config/input.ron");

    let game_data = GameDataBuilder::default()
        .with(CameraOrthoSystem::default(), "ortho_camera_system", &[])
        .with_bundle(HotReloadBundle::new(HotReloadStrategy::every(2)))?
        .with_bundle(TransformBundle::new())?
        .with_bundle(
            InputBundle::<StringBindings>::new().with_bindings_from_file(&key_bindings_path)?,
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
            system::movement::MovementSystem::default(),
            "movement_system",
            &["input_system"],
        )
        .with(Processor::<sanity_lib::assets::Pairs>::new(), "", &[])
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config)?
                        .with_clear([0.34, 0.36, 0.52, 1.0]),
                )
                .with_plugin(RenderUi::default())
                .with_plugin(RenderFlat2D::default())
                .with_plugin(RenderTiles2D::<sanity_lib::tile::RoomTile>::default()),
        )?;

    let mut game = Application::build(resources, state::room::RoomState::default())?
        .with_frame_limit(FrameRateLimitStrategy::Yield, 101)
        .build(game_data)?;

    game.run();

    Ok(())
}

/// A `RenderPlugin` for rendering a 2D Tiles entity.
#[derive(Clone, Default)]
pub struct RenderTiles2D<
    T: Tile,
    E: CoordinateEncoder = MortonEncoder2D,
    Z: DrawTiles2DBounds = DrawTiles2DBoundsDefault,
> {
    target: Target,
    _marker: PhantomData<(T, E, Z)>,
}

impl<T: Tile, E: CoordinateEncoder, Z: DrawTiles2DBounds> Debug for RenderTiles2D<T, E, Z> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl<T: Tile, E: CoordinateEncoder, Z: DrawTiles2DBounds> RenderTiles2D<T, E, Z> {
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
use amethyst::ecs::{Entities, Join, Read, ReadExpect, System, SystemData, World};
use amethyst::renderer::RenderGroupDesc;
impl<B: Backend, T: Tile, E: CoordinateEncoder, Z: DrawTiles2DBounds> RenderPlugin<B>
    for RenderTiles2D<T, E, Z>
{
    fn on_build<'a, 'b>(
        &mut self,
        world: &mut World,
        builder: &mut DispatcherBuilder<'a, 'b>,
    ) -> Result<(), amethyst_error::Error> {
        SetupData::<T, E>::setup(world);

        Ok(())
    }

    fn on_plan(
        &mut self,
        plan: &mut RenderPlan<B>,
        _factory: &mut Factory<B>,
        _res: &World,
    ) -> Result<(), amethyst_error::Error> {
        plan.extend_target(self.target, |ctx| {
            ctx.add(
                RenderOrder::Transparent,
                DrawTiles2DDesc::<T, E, Z>::default().builder(),
            )?;
            Ok(())
        });
        Ok(())
    }
}
